use crate::GpxSource;
use chrono::Duration;
use geo_types::Point;
use gpx::{TrackSegment, Waypoint};

extern crate gpx;
#[derive(Debug)]
pub struct Segment {
    segment: TrackSegment,
}

impl GpxSource<TrackSegment, Segment> for Segment {
    fn new(t: &TrackSegment) -> Segment {
        Segment { segment: t.clone() }
    }

    fn length_2d(&self) -> f64 {
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
    fn uphill_downhill(&self) -> (f64, f64) {
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

    fn duration(&self) -> i64 {
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
    fn points(&self) -> Vec<Waypoint> {
        self.points()
    }
}
