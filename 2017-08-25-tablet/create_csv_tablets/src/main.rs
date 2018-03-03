extern crate expected_value;

use std::env;
use std::num::ParseFloatError;
use std::str::FromStr;
use std::fs::File;
use std::io::Write;
use expected_value::expected_days_float;


struct Config {
    proportion: f64,
    output_file: String,
}

enum Error {
    ParseFloat,
    Args,
    Io,
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Error {
        Error::ParseFloat
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
        proportion: f64::from_str(&arguments.next().ok_or(Error::Args)?)?,
        output_file: arguments.next().ok_or(Error::Args)?,
    })
}

fn run(config: Config) -> Result<(), Error> {
    let mut file = File::create(config.output_file)?;
    file.write_all(b"Tablets, Expected Value\n")?;

    let mut point: u64;
    let mut value: f64;

    for k in 1..10_000 {
        point = k;
        value = expected_days_float(point, config.proportion);
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
