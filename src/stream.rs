use crate::coord;
use std::{io, io::prelude::*};

pub fn parse_stdin() -> Vec<[f64; 3]> {
    let mut coords = vec![];
    for line in io::stdin().lock().lines() {
        let coord: Vec<f64> = line
            .unwrap()
            .split(',')
            .map(|s| s.to_string().parse::<f64>().unwrap())
            .collect();
        let coord = coord::from_epsg_4326_to_3857(&[coord[0], coord[1], coord[2]]);
        coords.push(coord);
    }
    coords
}
