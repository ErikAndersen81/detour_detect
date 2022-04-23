use std::str::FromStr;

use crate::Args;
use clap::{Arg, Command};

pub fn handle_args() -> Args {
    let matches = Command::new("Detour Detection")
        .version("0.1.0")
        .author("Erik Andersen <3rik4ndersen@gmail.com>")
        .about("Runs the Detour Detect algorithm")
        .arg(
            Arg::new("in_file")
                .short('i')
                .long("input")
                .takes_value(true)
                .help("Input file. Should point to `graph.json`."),
        )
        .arg(
            Arg::new("evaluation_data")
                .short('e')
                .long("evaluation")
                .takes_value(true)
                .help("Evaluation data. Synthetic data as csv file"),
        )
	.arg(
            Arg::new("threshold")
                .short('t')
                .long("threshold")
                .takes_value(true)
                .help("Maximal distance in meters from any regular trajectory before a point is considered an outlier. Defaults to 50."),
        ).arg(
            Arg::new("sensitivity")
                .short('s')
                .long("sensitivity")
                .takes_value(true)
                .help("Number of outliers in a row before the alarm is raised. Defaults to 1."),
        )
        .get_matches();

    let training_file = if let Some(in_file) = matches.value_of("in_file") {
        //println!("Reading training data from {}", in_file);
        String::from(in_file)
    } else {
        println!("No input file specified!");
        String::from("")
    };
    let eval_file = if let Some(eval) = matches.value_of("evaluation_data") {
        //println!("Reading evalutaion data from {}", eval);
        Some(String::from(eval))
    } else {
        None
    };
    let threshold = if let Some(threshold) = matches.value_of("threshold") {
        let threshold = String::from_str(threshold).unwrap();
        threshold.parse::<f64>().unwrap()
    } else {
        50.0
    };
    let sensitivity = if let Some(sensitivity) = matches.value_of("sensitivity") {
        let sensitivity = String::from_str(sensitivity).unwrap();
        sensitivity.parse::<u32>().unwrap()
    } else {
        1
    };
    Args {
        training_file,
        eval_file,
        threshold,
        sensitivity,
    }
}
