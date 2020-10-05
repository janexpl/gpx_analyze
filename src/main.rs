extern crate chrono;
extern crate clap;
extern crate geo_types;
extern crate gpx;
use chrono::{DateTime, Duration, Utc};
use clap::{App, Arg};
use geo_types::Point;
use gpx::read;
use gpx::{Gpx, Track, TrackSegment, Waypoint};
use std::fs::File;
use std::io::BufReader;
#[derive(Debug)]
struct GpxFiles<'a> {
    file1: &'a GpxFile,
    file2: &'a GpxFile,
    percent: f64,
    tolerance: f64,
}

impl<'a> GpxFiles<'a> {
    fn new(file1: &'a GpxFile, file2: &'a GpxFile, percent: f64, tolerance: f64) -> Self {
        Self {
            file1,
            file2,
            percent,
            tolerance,
        }
    }

    fn compare(&self) {
        let mut i: u64 = 0;
        let mut j: u64 = 0;
        let mut found_point: u64 = 0;
        let mut start_point: Point<f64> = (0.00, 0.00).into();
        let mut end_point: Point<f64> = (0.00, 0.00).into();
        let mut success_points: u64 = 0;
        for pt1 in self.file1.segment.points.iter() {
            let lat_source = pt1.point().lat();
            let lon_source = pt1.point().lng();
            for pt2 in self.file2.segment.points.iter() {
                if j >= found_point {
                    let lat_client = pt2.point().lat();
                    let lon_client = pt2.point().lng();

                    let diff_lat = (lat_client - lat_source).abs();
                    let diff_lon = (lon_client - lon_source).abs();

                    if diff_lat * diff_lat + diff_lon * diff_lon <= self.tolerance * self.tolerance
                    {
                        found_point = j;
                        if i == 0 {
                            start_point = pt2.point();
                        }
                        if i == self.file1.segment.points.len() as u64 - 1 {
                            end_point = pt2.point();
                        }
                        success_points = success_points + 1;
                        break;
                    }
                }
                j = j + 1;
            }
            i = i + 1;
        }
        println!("Success points: {}", success_points);
        if self.file1.segment.points.len() as f64 * self.percent <= success_points as f64 {
            println!("Success points: {}", success_points);
            println!("Start point: {:?}", start_point);
            println!("End point: {:?}", end_point);
        }
    }
}
#[derive(Debug)]
struct GpxFile {
    gpx: Gpx,
    track: Track,
    segment: TrackSegment,
}

impl GpxFile {
    fn new(gpx: &Gpx) -> GpxFile {
        GpxFile {
            gpx: gpx.clone(),
            track: gpx.tracks[0].clone(),
            segment: gpx.tracks[0].segments[0].clone(),
        }
    }
    fn length_2d(&self) -> f64 {
        let mut last_point: Point<f64> = (0.0, 0.0).into();
        let mut i: u64 = 0;
        let mut dist: f64 = 0.00;
        for segs in self.segment.points.iter() {
            if i != 0 {
                dist = dist + self.distance(last_point, segs.point());
            }
            last_point = segs.point();
            i = i + 1;
        }
        dist
    }
    fn distance(&self, point1: Point<f64>, point2: Point<f64>) -> f64 {
        let r: f64 = 6371.00;
        let (longitude1, latitude1): (f64, f64) = point1.to_radians().x_y();
        let (longitude2, latitude2): (f64, f64) = point2.to_radians().x_y();
        let diff_long = longitude2 - longitude1;
        let diff_lat = latitude2 - latitude1;
        let a: f64 = (diff_lat / 2.00).sin() * (diff_lat / 2.00).sin()
            + (diff_long / 2.00).sin()
                * (diff_long / 2.00).sin()
                * latitude1.cos()
                * latitude2.cos();

        let c = 2.00 * a.sqrt().atan2((1.00 - a).sqrt());
        r * c * 1000.00
    }
    fn duration(&self) -> Duration {
        let mut last_point: Waypoint = Waypoint::new(Point::new(0.0, 0.0));
        let mut i: u64 = 0;
        let mut duration: Duration = Duration::zero();
        for segs in self.segment.points.iter() {
            if i != 0 {
                duration = duration + self.time(&last_point, segs);
            }
            last_point = segs.clone();
            i = i + 1;
        }
        duration
    }

    fn time(&self, point1: &Waypoint, point2: &Waypoint) -> Duration {
        let time1: Option<DateTime<Utc>> = point1.time;
        let time2: Option<DateTime<Utc>> = point2.time;
        let time1 = match time1 {
            Some(a) => a,
            None => panic!("Nie ma znacznika czasu"),
        };
        let time2 = match time2 {
            Some(a) => a,
            None => panic!("Nie ma znacznika czasu"),
        };
        let duration: Duration = time2.signed_duration_since(time1);
        duration
    }
}
fn main() {
    let matches = App::new("GPX Compare")
        .version("1.0")
        .author("Janusz Kwiatkowski <janex@jnx.pl>")
        .arg(
            Arg::with_name("gpx_file_source")
                .short("s")
                .long("gpx_source")
                .value_name("FILE")
                .help("Set source gpx file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("gpx_file_client")
                .short("c")
                .long("gpx_client")
                .value_name("FILE")
                .help("Set client gpx file")
                .takes_value(true),
        )
        .get_matches();
    let gpx_file_source = matches.value_of("gpx_file_source").unwrap_or("track.gpx");
    let gpx_file_client = matches.value_of("gpx_file_client").unwrap_or("track.gpx");
    let file_source = File::open(gpx_file_source).unwrap();
    let file_client = File::open(gpx_file_client).unwrap();
    let reader_source = BufReader::new(file_source);
    let reader_client = BufReader::new(file_client);

    let gpx_source: Gpx = read::<_>(reader_source).unwrap();
    let gpx_client: Gpx = read(reader_client).unwrap();
    let gpx_file1 = GpxFile::new(&gpx_source);
    let gpx_file2 = GpxFile::new(&gpx_client);
    let gpx_compare = GpxFiles::new(&gpx_file1, &gpx_file2, 0.8, 0.0001);
    println!("Distance 1: {}", gpx_file1.length_2d());
    println!("Distance 2: {}", gpx_file2.length_2d());
    println!("Duration: {:?}", gpx_file2.duration());
    gpx_compare.compare();
}
