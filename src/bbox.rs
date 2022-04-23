use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Bbox {
    pub x1: f64,
    pub x2: f64,
    pub y1: f64,
    pub y2: f64,
    pub t1: f64,
    pub t2: f64,
}

impl Bbox {
    /// Returns true if `point` is contained in `Bbox`
    pub fn contains(&self, point: [f64; 3]) -> bool {
        let x = (self.x1..=self.x2).contains(&point[0]);
        let y = (self.y1..=self.y2).contains(&point[1]);
        let t = (self.t1..=self.t2).contains(&point[2]);
        x & y & t
    }

    pub fn spatially_contains(&self, point: [f64; 3]) -> bool {
        let x = (self.x1..=self.x2).contains(&point[0]);
        let y = (self.y1..=self.y2).contains(&point[1]);
        x & y
    }

    /// Determine if `Bbox` is temporally after `point`
    pub fn is_after(&self, point: [f64; 3]) -> bool {
        point[2] < self.t1
    }
}
