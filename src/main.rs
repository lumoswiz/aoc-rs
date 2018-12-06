extern crate chrono;
extern crate clap;
extern crate failure;
extern crate failure_derive;
extern crate lazy_static;
extern crate native_tls;
extern crate paste;
extern crate regex;

mod client;
mod util;

use clap::{App, Arg};
use paste::item;
use std::fmt::Display;
use std::str::FromStr;
use std::time::Instant;

use client::Client;

fn main() {
    let matches = App::new("aoc")
        .version("0.1")
        .about("Execute Advent of Code problems")
        .author("Nicholas Lordello")
        .arg(
            Arg::with_name("year")
                .short("y")
                .long("year")
                .value_name("YEAR")
                .default_value("2018")
                .takes_value(true)
                .validator(validate::<i32>),
        )
        .arg(
            Arg::with_name("show-time")
                .long("show-time")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("days")
                .value_name("DAY")
                .required(true)
                .multiple(true)
                .validator(validate::<i32>),
        )
        .get_matches();

    let year: i32 = matches.value_of("year").unwrap().parse().unwrap();
    let show_time = matches.is_present("show-time");
    let days: Vec<i32> = matches
        .values_of("days")
        .unwrap()
        .map(|d| d.parse::<i32>().unwrap())
        .collect();

    let client = Client::from_env().expect("failed to create adventofcode.com client");
    for day in days {
        let input = client
            .get_input(year, day)
            .expect(&format!("failed to get input for {} day {}", year, day));

        let start = Instant::now();
        let answers = solve(year, day, &input);
        let time = Instant::now() - start;

        if show_time {
            let time = time.as_secs() as f64 + time.subsec_nanos() as f64 * 1e-9;
            println!("Day {} ({:.2}s)", day, time);
        } else {
            println!("Day {}", day);
        }

        println!("  puzzle 1: {}", answers.0);
        println!("  puzzle 2: {}", answers.1);
    }
}

fn validate<V: FromStr>(s: String) -> Result<(), String>
where
    V: FromStr,
    V::Err: Display,
{
    match V::from_str(&s) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

macro_rules! advent {
    ($year:tt {
        $(day $day:tt,)*
    }) => {
        item! {
            mod [<y $year>] {
                $(
                    pub mod [<d $day>];
                )*
            }

            fn solve(year: i32, day: i32, input: &str) -> (String, String) {
                match (year, day) {
                    $(
                        ($year, $day) => (
                            [<y $year>]::[<d $day>]::puzzle1(input).to_string(),
                            [<y $year>]::[<d $day>]::puzzle2(input).to_string(),
                        ),
                    )*
                    (y, d) => panic!("failed to get input for {} day {}", y, d),
                }
            }
        }
    };
}

advent!(
    2018 {
        day 01,
        day 02,
        day 03,
        day 04,
        day 05,
    }
);
