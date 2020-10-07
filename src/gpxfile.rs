extern crate chrono;
extern crate clap;
extern crate geo_types;
extern crate gpx;
use crate::{gpxsource::*, segments::*};

use gpx::Gpx;
#[derive(Debug)]
pub struct GpxFile {
    pub gpx: Gpx,
    // pub track: Track,
    // pub segments: Vec<Segment>,
}

impl GpxSource<Gpx, GpxFile> for GpxFile {
    fn new(gpx: &Gpx) -> GpxFile {
        // let mut segs: Vec<Segment> = Vec::new();
        // for segment in gpx.tracks[0].segments.iter() {
        //     let s = Segment::new(&segment);
        //     segs.push(s);
        // }
        GpxFile {
            gpx: gpx.clone(),
            // track: gpx.tracks[0].clone(),
            // segments: segs,
        }
    }
    fn length_2d(&self) -> f64 {
        let mut length: f64 = 0.00;
        for seg in self.gpx.tracks[0].segments.iter() {
            let s = Segment::new(seg);
            length = length + s.length_2d();
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
        let mut duration: i64;
        for seg in self.gpx.tracks[0].segments.iter() {
            let s = Segment::new(seg);
            duration = duration + s.duration();
        }
        duration
    }
}
