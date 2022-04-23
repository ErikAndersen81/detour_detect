use crate::{coord, Bbox};
use petgraph::stable_graph::StableDiGraph;
use std::fs::File;
use std::io::Read;

pub type Graph = StableDiGraph<(u32, Bbox), (u32, Vec<[f64; 3]>)>;

pub fn get_graph() -> Graph {
    let file = crate::ARGS.training_file.clone();
    let graph = read_graph(file);
    if let Ok(graph) = graph {
        graph
    } else {
        panic!()
    }
}

fn read_graph(file: String) -> std::io::Result<Graph> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let result: Result<Graph, serde_json::Error> = serde_json::from_str(contents.as_str());
    if let Ok(graph) = result {
        Ok(graph)
    } else {
        if let Some(err) = result.err() {
            println!("Error type: {:?}", err.classify());
            println!("Error column: {:?}", err.column());
            let mut out = err.to_string();
            out.replace_range(100..out.len() - 100, "");
            println!("Error description: {}", out);
        }
        panic!();
    }
}

pub type PointRecord = ([f64; 3], bool);

pub fn get_training_data(file: String) -> Vec<PointRecord> {
    read_synthetic(file).unwrap()
}

fn read_synthetic(file: String) -> std::io::Result<Vec<PointRecord>> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut lines = contents.lines();
    assert_eq!(lines.next().unwrap(), ",latitude,longitude,uid,anom_start");
    let records = lines
        .map(|line| {
            let mut fields = line.split(',');
            let time = fields.next();
            let lat = fields.next();
            let lon = fields.next();
            let is_out = fields.nth(1);
            match (lat, lon, time, is_out) {
                (Some(lat), Some(lon), Some(time), Some(is_out)) => {
                    let x: f64 = lat.parse().unwrap();
                    let y: f64 = lon.parse().unwrap();
                    let mut time = time.split(' ');
                    let mut time = time.nth(1).unwrap().split(':');
                    let t: f64 = match [time.next(), time.next(), time.next()] {
                        [Some(hour), Some(minutes), Some(seconds)] => {
                            let mut seconds = seconds.split('.');
                            let s: f64 = seconds.next().unwrap().parse().unwrap();
                            let ms: f64 = if let Some(ms) = seconds.next() {
                                ms.parse().unwrap()
                            } else {
                                0.0
                            };
                            match [hour.parse::<f64>(), minutes.parse::<f64>()] {
                                [Ok(hours), Ok(minutes)] => {
                                    (hours * 60.0 * 60.0 * 1000.0)
                                        + (minutes * 60.0 * 1000.0)
                                        + (s * 1000.0)
                                        + ms
                                }
                                _ => f64::NAN,
                            }
                        }
                        _ => f64::NAN,
                    };
                    let is_out: bool = if is_out == "False" {
                        false
                    } else if is_out == "True" {
                        true
                    } else {
                        panic!("cant parse bool!")
                    };
                    let coord = coord::from_epsg_4326_to_3857(&[x, y, t]);
                    (coord, is_out)
                }
                _ => panic!("parse error"),
            }
        })
        .collect::<Vec<PointRecord>>();
    Ok(records)
}
