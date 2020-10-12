use std::fmt;

use chrono::{DateTime, Duration, Utc};
use geo_types::Point;
use gpx::Waypoint;
pub struct TimeError;
impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to parse datetime") // user-facing output
    }
}

// Implement std::fmt::Debug for TimeError
impl fmt::Debug for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
pub trait GpxSource<T, U> {
    fn new(_: &T) -> U;
    fn length_2d(&self) -> f64;
    fn length_3d(&self) -> f64;
    fn uphill_downhill(&self) -> (f64, f64);
    fn duration(&self) -> i64;
    fn max_speed(&self) -> f64;
    fn points(&self) -> Vec<Waypoint>;
    fn height_difference(&self, point1: &Waypoint, point2: &Waypoint) -> f64 {
        let height1 = point1.elevation.unwrap();
        let height2 = point2.elevation.unwrap();
        height2 - height1
    }
    fn time(&self, point1: &Waypoint, point2: &Waypoint) -> Result<Duration, TimeError> {
        let time1: Option<DateTime<Utc>> = point1.time;
        let time2: Option<DateTime<Utc>> = point2.time;
        let time1 = match time1 {
            Some(a) => a,
            None => return Err(TimeError),
        };
        let time2 = match time2 {
            Some(a) => a,
            None => return Err(TimeError),
        };
        let duration: Duration = time2.signed_duration_since(time1);
        Ok(duration)
    }
    fn averge_speed(&self) -> f64 {
        let distance = self.length_3d();
        let time = self.duration() as f64;
        distance / time
    }
    fn speed_between(&self, point1: &Waypoint, point2: &Waypoint) -> f64 {
        let time = self.time(point1, point2).unwrap();
        let distance = self.distance(point1.point(), point2.point());
        let speed = distance / time.num_seconds() as f64;
        speed
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
}
