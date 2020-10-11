extern crate chrono;
extern crate clap;
extern crate geo_types;
extern crate gpx;
use crate::{gpxsource::*, segments::*};

use gpx::{Gpx, Waypoint};
#[derive(Debug)]
pub struct GpxFile {
    pub gpx: Gpx,
    // pub track: Track,
    pub segments: Vec<Segment>,
    pub points: Vec<Waypoint>,
}

impl GpxSource<Gpx, GpxFile> for GpxFile {
    fn new(gpx: &Gpx) -> GpxFile {
        let mut segs: Vec<Segment> = Vec::new();
        let mut pt: Vec<Waypoint> = Vec::new();
        for segment in gpx.tracks[0].segments.iter() {
            let s = Segment::new(&segment);
            segs.push(s);
        }
        for point in segs.iter() {
            let mut p = point.points();
            pt.append(&mut p);
        }

        GpxFile {
            gpx: gpx.clone(),
            // track: gpx.tracks[0].clone(),
            segments: segs,
            points: pt,
        }
    }
    fn length_2d(&self) -> f64 {
        let mut length: f64 = 0.00;
        for seg in self.segments.iter() {
            length = length + seg.length_2d();
        }
        length
    }
    fn uphill_downhill(&self) -> (f64, f64) {
        let mut uphill: f64 = 0.00;
        let mut downhill: f64 = 0.00;
        for seg in self.gpx.tracks[0].segments.iter() {
            let s = Segment::new(seg);
            let (up, down) = s.uphill_downhill();
            uphill = uphill + up;
            downhill = downhill + down;
        }
        (uphill, downhill)
    }
    fn duration(&self) -> i64 {
        let mut duration: i64 = 0;
        for seg in self.gpx.tracks[0].segments.iter() {
            let s = Segment::new(seg);
            duration = duration + s.duration();
        }
        duration
    }
    fn points(&self) -> Vec<Waypoint> {
        let mut points: Vec<Waypoint> = Vec::new();
        for seg in self.segments.iter() {
            let mut p = seg.points();
            points.append(&mut p);
        }
        points.clone()
    }

    fn length_3d(&self) -> f64 {
        let mut length: f64 = 0.00;
        for seg in self.segments.iter() {
            length = length + seg.length_3d();
        }
        length
    }
}
