use crate::util;
use failure::Error;
use lazy_static::lazy_static;
use nalgebra::Point3;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct Nanobot(Point3<i64>, i64);

impl Nanobot {
    fn is_in_range(&self, other: &Nanobot) -> bool {
        manhattan_distance(self.0, other.0) <= self.1
    }
}

lazy_static! {
    static ref NANOBOT_PATTERN: Regex =
        Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
}

impl FromStr for Nanobot {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = NANOBOT_PATTERN
            .captures(s)
            .ok_or_else(|| failure::err_msg("does not match nanobot pattern"))?;

        Ok(Nanobot(
            Point3::new(c[1].parse()?, c[2].parse()?, c[3].parse()?),
            c[4].parse()?,
        ))
    }
}

fn manhattan_distance(a: Point3<i64>, b: Point3<i64>) -> i64 {
    let c = a - b;
    c[0].abs() + c[1].abs() + c[2].abs()
}

pub fn puzzle1(input: &str) -> usize {
    let nanobots = util::parse::<Nanobot>(input).collect::<Vec<_>>();
    let strongest = nanobots.iter().max_by_key(|n| n.1).expect("no nanobots");

    nanobots.iter().filter(|n| strongest.is_in_range(n)).count()
}

pub fn puzzle2(input: &str) -> String {
    let nanobots = util::parse::<Nanobot>(input).collect::<Vec<_>>();

    "".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        const INPUT: &str = r"
            pos=<0,0,0>, r=4
            pos=<1,0,0>, r=1
            pos=<4,0,0>, r=3
            pos=<0,2,0>, r=1
            pos=<0,5,0>, r=3
            pos=<0,0,3>, r=1
            pos=<1,1,1>, r=1
            pos=<1,1,2>, r=1
            pos=<1,3,1>, r=1
        ";

        assert_eq!(super::puzzle1(INPUT), 7);
    }

    #[test]
    fn puzzle2() {
        const INPUT: &str = r"
            pos=<10,12,12>, r=2
            pos=<12,14,12>, r=2
            pos=<16,12,12>, r=4
            pos=<14,14,14>, r=6
            pos=<50,50,50>, r=200
            pos=<10,10,10>, r=5
        ";

        assert_eq!(super::puzzle2(INPUT), "12,12,12");
    }
}
