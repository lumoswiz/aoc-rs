use crate::util;
use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::fmt::{self, Debug, Formatter};
use std::ops::RangeInclusive;
use std::str::{self, FromStr};

#[derive(Debug)]
struct Vein {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
}

impl Vein {
    fn iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let ys = self.y.clone();
        self.x
            .clone()
            .map(move |x| ys.clone().map(move |y| (x, y)))
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

struct Grid {
    bounds: (usize, usize, usize, usize),
    size: (usize, usize),
    count_start: usize,
    fountain: (usize, usize),
    data: Vec<u8>,
}

impl Grid {
    fn new(veins: &[Vein], fountain: (usize, usize)) -> Grid {
        let (bounds, ymin) = veins.iter().fold(
            (
                (fountain.0, fountain.0, fountain.1, fountain.1),
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
        let size = (bounds.1 - bounds.0 + 1, bounds.3 - bounds.2 + 1);
        let count_start = ymin * size.0;
        let data = vec![b'.'; size.0 * size.1];

        let mut grid = Grid {
            bounds,
            size,
            count_start,
            fountain,
            data,
        };

        *grid.get_mut(fountain) = b'+';
        for vein in veins.iter() {
            for pos in vein.iter() {
                *grid.get_mut(pos) = b'#';
            }
        }

        grid
    }

    fn index_of(&self, (x, y): (usize, usize)) -> usize {
        let (xmin, _, ymin, _) = self.bounds;
        let (w, _) = self.size;

        (x - xmin) + (y - ymin) * w
    }

    fn get(&self, pos: (usize, usize)) -> u8 {
        let index = self.index_of(pos);
        self.data[index]
    }

    fn get_mut(&mut self, pos: (usize, usize)) -> &mut u8 {
        let index = self.index_of(pos);
        &mut self.data[index]
    }

    fn flow(&mut self) -> (usize, usize) {
        let mut falling = vec![self.fountain];

        while let Some(pos) = falling.pop() {
            let next_pos = (pos.0, pos.1 + 1);
            if next_pos.1 > self.bounds.3 {
                continue;
            }

            match self.get(next_pos) {
                b'.' => {
                    *self.get_mut(next_pos) = b'|';
                    falling.push(next_pos);
                }
                b'|' => continue,
                b'#' | b'~' => {
                    let mut start = pos.0;
                    let mut end = pos.0;
                    let mut closed = (false, false);
                    for i in 1.. {
                        let p = (pos.0 - i, pos.1);
                        match (self.get((p.0, p.1)), self.get((p.0, p.1 + 1))) {
                            (b'#', _) => {
                                closed.0 = true;
                                start = p.0 + 1;
                                break;
                            }
                            (_, b'#') | (_, b'~') => continue,
                            (_, b'.') | (_, b'|') => {
                                start = p.0;
                                break;
                            }
                            _ => unreachable!(),
                        }
                    }
                    for i in 1.. {
                        let p = (pos.0 + i, pos.1);
                        match (self.get((p.0, p.1)), self.get((p.0, p.1 + 1))) {
                            (b'#', _) => {
                                closed.1 = true;
                                end = p.0 - 1;
                                break;
                            }
                            (_, b'#') | (_, b'~') => continue,
                            (_, b'.') | (_, b'|') => {
                                end = p.0;
                                break;
                            }
                            _ => unreachable!(),
                        }
                    }
                    let fill = match closed {
                        (true, true) => {
                            let prev_pos = (pos.0, pos.1 - 1);
                            falling.push(prev_pos);
                            b'~'
                        }
                        (left, right) => {
                            if !left {
                                falling.push((start, pos.1));
                            }
                            if !right {
                                falling.push((end, pos.1));
                            }
                            b'|'
                        }
                    };
                    for x in start..=end {
                        let p = (x, pos.1);
                        *self.get_mut(p) = fill;
                    }
                }
                _ => unreachable!(),
            }
        }

        self.data[self.count_start..].iter().fold((0, 0), |(s, r), b| match *b {
            b'~' => (s + 1, r),
            b'|' => (s, r + 1),
            _ => (s, r),
        })
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (w, h) = self.size;
        for j in 0..h {
            let slice = &self.data[w * j..w * (j + 1)];
            let s = unsafe { str::from_utf8_unchecked(slice) };
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

pub fn puzzle1(input: &str) -> usize {
    let veins = util::parse::<Vein>(input).collect::<Vec<_>>();
    let mut grid = Grid::new(&veins, (500, 0));
    let (still, running) = grid.flow();

    still + running
}

pub fn puzzle2(input: &str) -> usize {
    let veins = util::parse::<Vein>(input).collect::<Vec<_>>();
    let mut grid = Grid::new(&veins, (500, 0));
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
