#[macro_use]
extern crate lazy_static;

pub mod arguments;
mod bbox;
pub mod coord;
mod edge;
mod outlier_detection;
mod parse;
mod stream;
use crate::outlier_detection::Detector;
pub use bbox::Bbox;
pub use edge::Trajectory;
pub use parse::Graph;

lazy_static! {
    pub static ref ARGS: Args = arguments::handle_args();
}

pub struct Args {
    pub training_file: String,
    pub eval_file: Option<String>,
    pub threshold: f64,
    pub sensitivity: u32,
}

fn main() {
    let graph = parse::get_graph();
    let graph = Detector::new(graph);
    let coords = if let Some(eval) = ARGS.eval_file.clone() {
        parse::get_training_data(eval)
    } else {
        panic!("no training data");
    };
    let mut track_anomaly = None;
    let mut anomaly_detected = None;
    let mut last_point_was_outlier = 0;
    for (point, outlier_truth) in coords {
        if outlier_truth {
            track_anomaly = Some(point);
        }
        match graph.is_outlier(point) {
            true => {
                println!("{:?}", point);
                if anomaly_detected.is_none() & (last_point_was_outlier > ARGS.sensitivity) {
                    anomaly_detected = Some(point);
                }
                last_point_was_outlier += 1;
            }
            false => {
                last_point_was_outlier = 0;
            }
        }
    }
    println!("s:{}, t:{}", ARGS.sensitivity, ARGS.threshold);
    print!("{}: ", ARGS.training_file.clone());
    match (track_anomaly, anomaly_detected) {
        (Some(anom), Some(pred)) => println!(
            "TP anom:{:?} pred:{:?} \t({:.0}ms)",
            anom,
            pred,
            pred[2] - anom[2]
        ),
        (Some(anom), None) => println!("FN {:?}", anom),
        (None, Some(pred)) => println!("FP {:?}", pred),
        (None, None) => println!("TN"),
    }
}
