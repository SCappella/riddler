extern crate expected_value;

use std::env;
use std::num::{ParseIntError, ParseFloatError};
use std::str::FromStr;
use expected_value::expected_days_float;

#[derive(Default)]
struct Config {
    number_of_tablets: u64,
    proportion: f64,
}

enum Error {
    ParseInt,
    ParseFloat,
    Args,
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Error {
        Error::ParseInt
    }
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Error {
        Error::ParseFloat
    }
}

fn parse_args() -> Result<Config, Error> {
    let mut arguments = env::args();
    arguments.next(); // remove command name
    let mut config = Config::default();

    config.number_of_tablets = u64::from_str(&arguments.next().ok_or(Error::Args)?)?;

    config.proportion = f64::from_str(&arguments.next().ok_or(Error::Args)?)?;

    Ok(config)
}

fn main() {
    if let Ok(config) = parse_args() {
        println!(
            "{}",
            expected_days_float(config.number_of_tablets, config.proportion)
        );
    } else {
        println!("Invalid arguments");
    }
}
