extern crate clap;
extern crate failure;
extern crate failure_derive;
extern crate native_tls;

mod client;
mod y2018;

use clap::{App, Arg};
use std::fmt::Display;
use std::str::FromStr;

use client::Client;

fn main() {
    let matches = App::new("aoc")
        .version("0.1")
        .about("Execute Advent of Code problems")
        .author("Nicholas Lordello")
        .arg(
            Arg::with_name("year")
                .value_name("YEAR")
                .required(true)
                .validator(validate::<i32>),
        ).arg(
            Arg::with_name("days")
                .value_name("DAY")
                .required(true)
                .multiple(true)
                .validator(validate::<i32>),
        ).get_matches();

    let year: i32 = matches.value_of("year").unwrap().parse().unwrap();
    let days: Vec<i32> = matches
        .values_of("days")
        .unwrap()
        .map(|d| d.parse::<i32>().unwrap())
        .collect();

    let client = Client::from_env().expect("failed to create adventofcode.com client");
    for day in days {
        let date = format!("{}/12/{}", day, year);
        let input = client
            .get_input(year, day)
            .expect(&format!("failed to get input for {}", date));
        let result = match (year, day) {
            (2018, 1) => (
                y2018::d01::run1(&input).to_string(),
                y2018::d01::run2(&input).to_string(),
            ),
            _ => unimplemented!(),
        };

        println!("{}:", date);
        println!(" - {}", result.0);
        println!(" - {}", result.1);
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
