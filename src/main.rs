mod client;

use crate::client::Client;
use clap::{Arg, Command};
use std::time::Instant;

fn main() {
    let matches = Command::new("aoc")
        .version("0.1")
        .about("Execute Advent of Code problems")
        .author("Nicholas Lordello")
        .arg(
            Arg::new("year")
                .short('y')
                .long("year")
                .value_name("YEAR")
                .default_value("2023")
                .value_parser(clap::value_parser!(i32)), // Replace validator()
        )
        .arg(Arg::new("show-time").long("show-time"))
        .arg(
            Arg::new("days")
                .short('d') // Short flags now use single quotes
                .value_name("DAY")
                .required(true)
                .num_args(1..)
                .value_parser(clap::value_parser!(i32)), // Replace validator()
        )
        .get_matches();

    let year: i32 = *matches.get_one::<i32>("year").expect("year is required");
    let show_time = matches.get_flag("show-time");
    let days: Vec<i32> = matches
        .get_many::<i32>("days")
        .unwrap_or_default()
        .copied() // `get_many()` returns references, so we use `.copied()`
        .collect();

    let client = Client::from_env().expect("failed to create adventofcode.com client");
    for day in days {
        let input = client
            .get_input(year, day)
            .unwrap_or_else(|_| panic!("failed to get input for {} day {}", year, day));

        let start = Instant::now();
        let answers = solve(year, day, &input);
        let time = Instant::now() - start;

        if show_time {
            let time = time.as_secs() as f64 + f64::from(time.subsec_nanos()) * 1e-9;
            println!("Day {} ({:.2}s)", day, time);
        } else {
            println!("Day {}", day);
        }

        println!("  puzzle 1: {}", answers.0);
        println!("  puzzle 2: {}", answers.1);
    }
}

macro_rules! advent {
    ($(
        $year:tt {
            $($day:tt,)*
        },
    )*) => {
        $(
            mod $year {
                $(
                    pub mod $day;
                )*
            }
        )*

        fn solve(year: i32, day: i32, input: &str) -> (String, String) {
            let year_str = format!("year{}", year);
            let day_str = format!("day{:02}", day);

            match (year_str.as_str(), day_str.as_str()) {
                $(
                    $(
                        (stringify!($year), stringify!($day)) => (
                            $year::$day::puzzle1(input).to_string(),
                            $year::$day::puzzle2(input).to_string(),
                        ),
                    )*
                )*
                _ => panic!("failed to get input for year {} day {}", year, day),
            }
        }
    };
}

advent!(year2016 {
    day01,
    // day02,
    // day03,
    // day04,
},);
