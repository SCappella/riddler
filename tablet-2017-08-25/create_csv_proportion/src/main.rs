extern crate expected_value;

use std::env;
use std::num::ParseIntError;
use std::str::FromStr;
use std::fs::File;
use std::io::Write;
use expected_value::expected_days_float;


struct Config {
    number_of_tablets: u64,
    output_file: String,
}

enum Error {
    ParseInt,
    Args,
    Io,
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Error {
        Error::ParseInt
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Error {
        Error::Io
    }
}

fn parse_args() -> Result<Config, Error> {
    let mut arguments = env::args();
    arguments.next(); // remove command name
    Ok(Config {
        number_of_tablets: u64::from_str(&arguments.next().ok_or(Error::Args)?)?,
        output_file: arguments.next().ok_or(Error::Args)?,
    })
}

fn run(config: Config) -> Result<(), Error> {
    let mut file = File::create(config.output_file)?;
    file.write_all(b"Proportion, Expected Value\n")?;

    let mut point: f64;
    let mut value: f64;

    for k in 1..10_000 {
        point = 0.0001 * f64::from(k);
        value = expected_days_float(config.number_of_tablets, point);
        file.write_fmt(format_args!("{}, {}\n", point, value))?;
    }
    file.sync_all()?;
    Ok(())
}

fn main() {
    if let Ok(config) = parse_args() {
        if run(config).is_err() {
            println!("Error while running");
        }
    } else {
        println!("Invalid arugments");
    }
}
