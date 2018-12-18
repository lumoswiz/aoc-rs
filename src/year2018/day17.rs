use crate::util::{self, Grid};
use failure::Error;
use lazy_static::lazy_static;
use nalgebra::{Point2, Vector2};
use regex::Regex;
use std::cmp;
use std::ops::RangeInclusive;
use std::str::{self, FromStr};

#[derive(Debug)]
struct Vein {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
}

impl Vein {
    fn iter(&self) -> impl Iterator<Item = Point2<usize>> {
        let ys = self.y.clone();
        self.x
            .clone()
            .map(move |x| ys.clone().map(move |y| [x, y].into()))
            .flatten()
    }
}

lazy_static! {
    static ref VEIN_PATTERN: Regex = Regex::new(r"([xy])=(\d+), ([xy])=(\d+)\.\.(\d+)").unwrap();
}

impl FromStr for Vein {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = VEIN_PATTERN
            .captures(s)
            .ok_or_else(|| failure::err_msg("does not match vein pattern"))?;

        if captures[1] == captures[3] {
            return Err(failure::err_msg("missing x or y coordinates"));
        }

        let (a, b, c) = (
            captures[2].parse()?,
            captures[4].parse()?,
            captures[5].parse()?,
        );
        let (x, y) = match &captures[1] {
            "x" => (a..=a, b..=c),
            "y" => (b..=c, a..=a),
            _ => unreachable!(),
        };

        Ok(Vein { x, y })
    }
}

struct Ground {
    fountain: Point2<usize>,
    ymin: usize,
    grid: Grid,
}

impl Ground {
    fn new(veins: &[Vein], fountain: Point2<usize>) -> Ground {
        let (bounds, ymin) = veins.iter().fold(
            (
                (fountain[0], fountain[0], fountain[1], fountain[1]),
                usize::max_value(),
            ),
            |(bounds, ymin), vein| {
                (
                    (
                        cmp::min(*vein.x.start() - 1, bounds.0),
                        cmp::max(*vein.x.end() + 1, bounds.1),
                        cmp::min(*vein.y.start(), bounds.2),
                        cmp::max(*vein.y.end(), bounds.3),
                    ),
                    cmp::min(*vein.y.start(), ymin),
                )
            },
        );

        let w = bounds.1 - bounds.0 + 1;
        let h = bounds.3 - bounds.2 + 1;
        let origin = Vector2::new(bounds.0, bounds.2);

        let mut grid = Grid::new(w, h);

        grid[fountain - origin] = b'+';
        for vein in veins.iter() {
            for pos in vein.iter() {
                grid[pos - origin] = b'#';
            }
        }

        Ground {
            fountain: fountain - origin,
            ymin,
            grid,
        }
    }

    fn flow(&mut self) -> (usize, usize) {
        let mut falling = vec![self.fountain];

        while let Some(pos) = falling.pop() {
            let next_pos = pos + Vector2::y();
            let next = match self.grid.get(next_pos) {
                Some(next) => next,
                None => continue,
            };

            match next {
                b'.' => {
                    self.grid[next_pos] = b'|';
                    falling.push(next_pos);
                }
                b'|' => continue,
                b'#' | b'~' => {
                    let mut start = pos[0];
                    let mut end = pos[0];
                    let mut closed = (false, false);

                    for i in 1.. {
                        let offset = Vector2::new(i, 0);
                        let p = pos - offset;
                        match (self.grid[p], self.grid[p + Vector2::y()]) {
                            (b'#', _) => {
                                closed.0 = true;
                                start = p[0] + 1;
                                break;
                            }
                            (_, b'#') | (_, b'~') => continue,
                            (_, b'.') | (_, b'|') => {
                                start = p[0];
                                break;
                            }
                            _ => unreachable!(),
                        }
                    }
                    for i in 1.. {
                        let offset = Vector2::new(i, 0);
                        let p = pos + offset;
                        match (self.grid[p], self.grid[p + Vector2::y()]) {
                            (b'#', _) => {
                                closed.1 = true;
                                end = p[0] - 1;
                                break;
                            }
                            (_, b'#') | (_, b'~') => continue,
                            (_, b'.') | (_, b'|') => {
                                end = p[0];
                                break;
                            }
                            _ => unreachable!(),
                        }
                    }
                    let fill = match closed {
                        (true, true) => {
                            falling.push(pos - Vector2::y());
                            b'~'
                        }
                        (left, right) => {
                            if !left {
                                falling.push([start, pos[1]].into());
                            }
                            if !right {
                                falling.push([end, pos[1]].into());
                            }
                            b'|'
                        }
                    };
                    for x in start..=end {
                        self.grid[[x, pos[1]]] = fill;
                    }
                }
                _ => unreachable!(),
            }
        }

        self.grid
            .iter()
            .filter(|(pos, _)| pos[1] >= self.ymin)
            .fold((0, 0), |(s, r), (_, b)| match b {
                b'~' => (s + 1, r),
                b'|' => (s, r + 1),
                _ => (s, r),
            })
    }
}

pub fn puzzle1(input: &str) -> usize {
    let veins = util::parse::<Vein>(input).collect::<Vec<_>>();
    let mut grid = Ground::new(&veins, [500, 0].into());
    let (still, running) = grid.flow();

    still + running
}

pub fn puzzle2(input: &str) -> usize {
    let veins = util::parse::<Vein>(input).collect::<Vec<_>>();
    let mut grid = Ground::new(&veins, [500, 0].into());
    let (still, _) = grid.flow();

    still
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"
        x=495, y=2..7
        y=7, x=495..501
        x=501, y=3..7
        x=498, y=2..4
        x=506, y=1..2
        x=498, y=10..13
        x=504, y=10..13
        y=13, x=498..504
    ";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(INPUT), 57);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(INPUT), 29);
    }
}
