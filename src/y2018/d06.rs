use crate::util;
use failure::{self, Error};
use std::cmp;
use std::fmt::{self, Debug, Formatter};
use std::str::FromStr;

struct Coord(usize, usize);

impl FromStr for Coord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, ',').map(|p| p.trim());
        match (parts.next(), parts.next()) {
            (Some(x), Some(y)) => Ok(Coord(x.parse()?, y.parse()?)),
            _ => Err(failure::err_msg("expected 'x, y' format")),
        }
    }
}

struct Grid<T> {
    origin: (usize, usize),
    size: (usize, usize),
    squares: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    fn new((x0, y0, xmax, ymax): (usize, usize, usize, usize)) -> Grid<T> {
        let w = xmax - x0 + 1;
        let h = ymax - y0 + 1;

        println!("{:?}", (x0, y0, xmax, ymax, w, h));
        Grid {
            origin: (x0, y0),
            size: (w, h),
            squares: vec![T::default(); w * h],
        }
    }
}

impl<T> Grid<T> {
    fn contains(&self, x: usize, y: usize) -> bool {
        let (x0, y0) = self.origin;
        let (w, h) = self.size;

        x >= x0 && x < x0 + w && y >= y0 && y < y0 + h
    }

    fn index_of(&self, x: usize, y: usize) -> usize {
        let (x0, y0) = self.origin;
        let (w, _) = self.size;

        (x - x0) + (y - y0) * w
    }

    fn get(&self, x: usize, y: usize) -> &T {
        &self.squares[self.index_of(x, y)]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let i = self.index_of(x, y);
        &mut self.squares[i]
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (w, h) = self.size;

        let mut set = f.debug_set();
        for y in 0..h {
            let xs = &self.squares[y * w..(y + 1) * w];
            set.entry(&format_args!("{:?}", xs));
        }
        set.finish()
    }
}

pub fn puzzle1(input: &str) -> i64 {
    let mut coords = util::parse::<Coord>(input);
    let c = coords.next().unwrap();
    let coords = coords.collect::<Vec<_>>();

    let mut grid = Grid::<(char, usize)>::new(coords.iter().fold(
        (c.0, c.1, c.0, c.1),
        |(x0, y0, xmax, ymax), Coord(x, y)| {
            (
                cmp::min(x0, *x),
                cmp::min(y0, *y),
                cmp::max(xmax, *x),
                cmp::max(ymax, *y),
            )
        },
    ));

    for (i, coord) in coords.into_iter().enumerate() {
        let id = (b'A' + i as u8) as char;
        let Coord(x, y) = coord;
        *grid.get_mut(x, y) = (id, 0);
    }

    42
}

pub fn puzzle2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"
        1, 1
        1, 6
        8, 3
        3, 4
        5, 5
        8, 9
    ";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(INPUT), 0);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(INPUT), 0);
    }
}
