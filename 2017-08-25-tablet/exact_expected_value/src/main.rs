extern crate gmp;
extern crate expected_value;

use std::env;
use std::num::ParseIntError;
use gmp::mpq::Mpq;
use expected_value::expected_days_exact;

struct Config {
    number_of_tablets: u64,
    proportion: Mpq,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            number_of_tablets: 0_u64,
            proportion: Mpq::ratio(&0.into(), &1.into()),
        }
    }
}

enum Error {
    ParseIntError,
    ArgsError,
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Error {
        Error::ParseIntError
    }
}

fn parse_args() -> Result<Config, Error> {
    let mut arguments = env::args();
    arguments.next(); // remove command name
    let mut config = Config::default();

    config.number_of_tablets = u64::from_str_radix(&arguments.next().ok_or(Error::ArgsError)?, 10)?;

    let numerator = u64::from_str_radix(&arguments.next().ok_or(Error::ArgsError)?, 10)?;
    let denominator = u64::from_str_radix(&arguments.next().ok_or(Error::ArgsError)?, 10)?;
    config.proportion = Mpq::ratio(&numerator.into(), &denominator.into());

    Ok(config)
}

fn main() {
    if let Ok(config) = parse_args() {
        println!(
            "{}",
            expected_days_exact(config.number_of_tablets, config.proportion)
        );
    } else {
        println!("Invalid arguments");
    }
}
