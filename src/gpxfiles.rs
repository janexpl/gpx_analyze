use crate::gpxfile::*;
use geo_types::Point;
use gpx::{Gpx, Route, Track, TrackSegment, Waypoint};

#[derive(Debug)]
pub struct GpxFiles<'a> {
    file1: &'a GpxFile,
    file2: &'a GpxFile,
    percent: f64,
    pub tolerance: f64,
    step: f64,
    iterations: u32,
}

impl<'a> GpxFiles<'a> {
    pub fn new(
        file1: &'a GpxFile,
        file2: &'a GpxFile,
        percent: f64,
        tolerance: f64,
        step: f64,
        iterations: u32,
    ) -> Self {
        Self {
            file1,
            file2,
            percent,
            tolerance,
            step,
            iterations,
        }
    }
    pub fn best_fit(&mut self) -> GpxFile {
        let mut length: f64 = 0.00;
        let mut fitted_gpx: GpxFile = GpxFile::new(&self.file2.gpx);
        for _ in 0..self.iterations {
            let result = self.compare();
            match result {
                Some((a, b)) => {
                    let new_gpx = self.extract_gpx(a, b);
                    let new_length = new_gpx.length_2d();
                    if new_length > length {
                        length = new_length;
                        fitted_gpx = new_gpx;
                    }
                    self.tolerance = self.tolerance + self.step;
                }
                None => {}
            }
        }
        fitted_gpx
    }
    fn compare(&self) -> Option<(Point<f64>, Point<f64>)> {
        let mut found_point: u64 = 0;
        let mut start_point: Point<f64> = (0.00, 0.00).into();
        let mut end_point: Point<f64> = (0.00, 0.00).into();
        let mut success_points: u64 = 0;

        for (i, pt1) in self.file1.segment.points.iter().enumerate() {
            let lat_source = pt1.point().lat();
            let lon_source = pt1.point().lng();
            //let mut j: u64 = 1;

            for (j, pt2) in self.file2.segment.points.iter().enumerate() {
                if j as u64 > found_point {
                    let lat_client = pt2.point().lat();
                    let lon_client = pt2.point().lng();

                    let diff_lat = (lat_client - lat_source).abs();
                    let diff_lon = (lon_client - lon_source).abs();

                    if diff_lat * diff_lat + diff_lon * diff_lon <= self.tolerance * self.tolerance
                    {
                        found_point = j as u64;
                        if i == 0 {
                            start_point = pt2.point();
                        }
                        if i == self.file1.segment.points.len() - 1 {
                            end_point = pt2.point();
                        }
                        success_points = success_points + 1;
                        // println!("Found point: {}", found_point);
                        // println!(
                        //     "Source: {}:{}, Client: {}:{}",
                        //     lat_source, lon_source, lat_client, lon_client
                        // );
                        break;
                    }
                }
            }
        }

        if self.file1.segment.points.len() as f64 * self.percent <= success_points as f64 {
            None
        } else {
            Some((start_point, end_point))
        }
    }
    fn extract_gpx(&self, start_point: Point<f64>, end_point: Point<f64>) -> GpxFile {
        let mut extract: bool = false;
        let mut track = Track::new();
        let mut segment: TrackSegment = TrackSegment::new();
        for seg in self.file2.segment.points.iter() {
            // println!("{:?} {:?} {:?}", seg.point(), start_point, end_point);

            if seg.point().eq(&start_point) {
                extract = true;
            }
            if extract == true {
                segment.points.push(seg.clone());
            }
            if seg.point().eq(&end_point) {
                extract = false;
            }
        }
        track.segments.push(segment);
        let mut tracks: Vec<Track> = Vec::new();
        tracks.push(track);
        let waypoints: Vec<Waypoint> = Vec::new();
        let routes: Vec<Route> = Vec::new();
        let gpxf = Gpx {
            version: gpx::GpxVersion::Gpx11,
            tracks,
            metadata: None,
            waypoints: waypoints,
            routes: routes,
        };
        let gpx_file = GpxFile::new(&gpxf);
        gpx_file
    }
}
