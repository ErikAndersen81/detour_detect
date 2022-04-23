use crate::ARGS;

pub trait Trajectory {
    /// Determine if the point is spatially close enough the trajectory
    fn on_trj(&self, point: [f64; 3]) -> bool;
}

impl Trajectory for Vec<[f64; 3]> {
    fn on_trj(&self, point: [f64; 3]) -> bool {
        let mut min_dist = f64::INFINITY;
        for (i, &q) in self.iter().enumerate() {
            let p = [point[0], point[1]];
            let q1 = [q[0], q[1]];
            let dist = euclid(p, q1);
            min_dist = min_dist.min(dist);
            if dist < ARGS.threshold {
                return true;
            }
            if i + 1 < self.len() {
                let q2 = [self[i + 1][0], self[i + 1][1]];
                let dist = euclid(p, q2);
                min_dist = min_dist.min(dist);
                if dist < ARGS.threshold {
                    return true;
                }
                let line = Line::from(q1, q2);
                let dist = line.distance(point);
                min_dist = min_dist.min(dist);
                if dist < ARGS.threshold {
                    return true;
                }
            }
        }
        println!("min_dist: {} for point: {:?}", min_dist, point);
        false
    }
}

fn euclid(p: [f64; 2], q: [f64; 2]) -> f64 {
    let [px, py] = p;
    let [qx, qy] = q;
    ((px - qx).powi(2) + (py - qy).powi(2)).sqrt()
}

struct Line {
    pub start: [f64; 2],
    pub end: [f64; 2],
}

impl Line {
    /// Determines the intersection point of two line segments.
    fn intersection(&self, other: &Line) -> Option<[f64; 2]> {
        let (x1, y1, x2, y2) = (self.start[0], self.start[1], self.end[0], self.end[1]);
        let (x3, y3, x4, y4) = (other.start[0], other.start[1], other.end[0], other.end[1]);
        let denom: f64 = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        let t: f64 = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
        let u: f64 = ((x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2)) / denom;
        if (0.0..=1.0).contains(&t) & (0.0..=1.0).contains(&u) {
            Some([x1 + t * (x2 - x1), y1 + t * (y2 - y1)])
        } else {
            None
        }
    }

    /// Returns a vector orthogonal to `self`
    fn orthogonal(&self) -> [f64; 2] {
        let x = self.start[0] - self.end[0];
        let y = self.start[1] - self.end[1];
        [y, -x]
    }

    /// Creates a 2D-line from two 2D-coordinates
    pub fn from(a: [f64; 2], b: [f64; 2]) -> Self {
        let start = [a[0], a[1]];
        let end = [b[0], b[1]];
        Line { start, end }
    }

    /// Projects a point orthogonally onto `self`
    /// If the point is not on the line segment, returns None.
    /// Otherwise return the projected point on `self`.
    fn project_point(&self, point: [f64; 3]) -> Option<[f64; 2]> {
        // We need to ensure the line segment is long enough s.t. we will know if it intersects the
        // `self`-line segment, so we multiply the vector by threshold supplied by ARGS
        let [vx, vy] = self.orthogonal();
        let [vx, vy] = [vx * ARGS.threshold, vy * ARGS.threshold];
        let (start_x, start_y) = (point[0] - vx, point[1] - vy);
        let (end_x, end_y) = (point[0] + vx, point[1] + vy);
        let line = Line::from([start_x, start_y], [end_x, end_y]);
        self.intersection(&line)
    }

    pub fn distance(&self, point: [f64; 3]) -> f64 {
        if let Some(intersection_point) = self.project_point(point) {
            euclid([point[0], point[1]], intersection_point)
        } else {
            f64::INFINITY
        }
    }
}
