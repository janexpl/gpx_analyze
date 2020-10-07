extern crate chrono;
extern crate clap;
extern crate geo_types;
extern crate gpx;
use chrono::{DateTime, Duration, Utc};
use geo_types::Point;
use gpx::{Gpx, Track, TrackSegment, Waypoint};
#[derive(Debug)]
pub struct GpxFile {
    pub gpx: Gpx,
    pub track: Track,
    pub segment: TrackSegment,
}

impl GpxFile {
    pub fn new(gpx: &Gpx) -> GpxFile {
        GpxFile {
            gpx: gpx.clone(),
            track: gpx.tracks[0].clone(),
            segment: gpx.tracks[0].segments[0].clone(),
        }
    }
    pub fn length_2d(&self) -> f64 {
        let mut last_point: Point<f64> = (0.0, 0.0).into();

        let mut dist: f64 = 0.00;
        for (i, segs) in self.segment.points.iter().enumerate() {
            if i != 0 {
                dist = dist + self.distance(last_point, segs.point());
            }
            last_point = segs.point();
        }
        dist
    }
    pub fn uphill_downhill(&self) -> (f64, f64) {
        let mut last_point = &Waypoint::new(Point::new(0.00, 0.00));
        let mut uphill: f64 = 0.00;
        let mut downhill: f64 = 0.00;
        let mut difference: f64 = 0.00;
        for (i, seg) in self.segment.points.iter().enumerate() {
            if i == 0 {
                last_point = seg;
            } else {
                difference = self.height_difference(&last_point, seg);
                last_point = seg;
            }
            if difference > 0.00 {
                uphill = uphill + difference;
            } else if difference < 0.00 {
                downhill = downhill + difference;
            }
        }
        (uphill, downhill)
    }
    fn height_difference(&self, point1: &Waypoint, point2: &Waypoint) -> f64 {
        let height1 = point1.elevation.unwrap();
        let height2 = point2.elevation.unwrap();
        height2 - height1
    }
    pub fn distance(&self, point1: Point<f64>, point2: Point<f64>) -> f64 {
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
    pub fn duration(&self) -> i64 {
        let mut last_point: Waypoint = Waypoint::new(Point::new(0.0, 0.0));
        let mut duration: Duration = Duration::zero();
        for (i, segs) in self.segment.points.iter().enumerate() {
            if i != 0 {
                duration = duration + self.time(&last_point, segs);
            }
            last_point = segs.clone();
        }
        duration.num_seconds()
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
