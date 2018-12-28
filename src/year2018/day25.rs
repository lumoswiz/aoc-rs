use crate::util;
use failure::Error;
use nalgebra::Point4;
use std::str::FromStr;

struct P(Point4<i32>);

impl FromStr for P {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let mut next = || -> Result<i32, Error> {
            Ok(parts
                .next()
                .ok_or_else(|| failure::err_msg("wrong number of parts"))?
                .parse()?)
        };

        let result = Point4::new(next()?, next()?, next()?, next()?);
        if parts.next().is_some() {
            return Err(failure::err_msg("wrong number of parts"));
        }

        Ok(P(result))
    }
}

fn manhattan_distance(a: Point4<i32>, b: Point4<i32>) -> i32 {
    let c = a - b;
    c[0].abs() + c[1].abs() + c[2].abs() + c[3].abs()
}

pub fn puzzle1(input: &str) -> i32 {
    let mut points = util::parse::<P>(input).map(|p| p.0).collect::<Vec<_>>();

    let mut constallations = 0;
    let mut pending = Vec::with_capacity(points.len());
    while let Some(start) = points.pop() {
        constallations += 1;
        pending.push(start);
        while let Some(next) = pending.pop() {
            for i in (0..points.len()).rev() {
                if manhattan_distance(next, points[i]) <= 3 {
                    pending.push(points[i]);
                    points.swap_remove(i);
                }
            }
        }
    }

    constallations
}

pub fn puzzle2(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        assert_eq!(
            super::puzzle1(
                r"
                    0,0,0,0
                    3,0,0,0
                    0,3,0,0
                    0,0,3,0
                    0,0,0,3
                    0,0,0,6
                    9,0,0,0
                    12,0,0,0
                "
            ),
            2
        );

        assert_eq!(
            super::puzzle1(
                r"
                    -1,2,2,0
                    0,0,2,-2
                    0,0,0,-2
                    -1,2,0,0
                    -2,-2,-2,2
                    3,0,2,-1
                    -1,3,2,2
                    -1,0,-1,0
                    0,2,1,-2
                    3,0,0,0
                "
            ),
            4
        );

        assert_eq!(
            super::puzzle1(
                r"
                    1,-1,0,1
                    2,0,-1,0
                    3,2,-1,0
                    0,0,3,1
                    0,0,-1,-1
                    2,3,-2,0
                    -2,2,0,0
                    2,-2,0,-1
                    1,-1,0,-1
                    3,2,0,2
                "
            ),
            3
        );

        assert_eq!(
            super::puzzle1(
                r"
                    1,-1,-1,-2
                    -2,-2,0,1
                    0,2,1,3
                    -2,3,-2,1
                    0,2,3,-2
                    -1,-1,1,-2
                    0,-2,-1,0
                    -2,2,3,-1
                    1,2,2,0
                    -1,-2,0,-2
                "
            ),
            8
        );
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }
}
