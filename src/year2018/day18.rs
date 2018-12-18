use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::mem;
use std::str;

struct Grid {
    dim: (usize, usize),
    data: Vec<u8>,
    next: Vec<u8>,
}

impl Grid {
    fn new(layout: &str) -> Grid {
        let width = layout.trim().find('\n').unwrap();
        let data = layout
            .trim()
            .split('\n')
            .map(|l| l.trim().bytes())
            .flatten()
            .collect::<Vec<_>>();
        let height = data.len() / width;
        let dim = (width, height);
        let next = vec![b'.'; data.len()];

        Grid { dim, data, next }
    }

    fn index_of(&self, x: usize, y: usize) -> usize {
        x + y * self.dim.0
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        let i = self.index_of(x, y);
        self.data[i]
    }

    fn adjacent<'a>(&'a self, x: usize, y: usize) -> impl 'a + Iterator<Item = u8> {
        static ADJACENT: [(isize, isize); 8] = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];
        let (w, h) = self.dim;
        let (w, h) = (w as isize, h as isize);

        ADJACENT
            .iter()
            .map(move |(dx, dy)| ((x as isize) + dx, (y as isize) + dy))
            .filter(move |&(x, y)| x >= 0 && x < w && y >= 0 && y < h)
            .map(move |(x, y)| self.get(x as usize, y as usize))
    }

    fn generation(&mut self) {
        let (w, h) = self.dim;
        for x in 0..w {
            for y in 0..h {
                let (_, tree, yard) =
                    self.adjacent(x, y)
                        .fold((0, 0, 0), |(empty, tree, yard), b| match b {
                            b'.' => (empty + 1, tree, yard),
                            b'|' => (empty, tree + 1, yard),
                            b'#' => (empty, tree, yard + 1),
                            _ => (empty, tree, yard),
                        });

                let curr = self.get(x, y);
                let i = self.index_of(x, y);
                self.next[i] = match curr {
                    b'.' => {
                        if tree >= 3 {
                            b'|'
                        } else {
                            b'.'
                        }
                    }
                    b'|' => {
                        if yard >= 3 {
                            b'#'
                        } else {
                            b'|'
                        }
                    }
                    b'#' => {
                        if tree > 0 && yard > 0 {
                            b'#'
                        } else {
                            b'.'
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        mem::swap(&mut self.data, &mut self.next);
    }

    fn count(&self) -> usize {
        let (tree, yard) = self.data.iter().fold((0, 0), |(tree, yard), b| match *b {
            b'|' => (tree + 1, yard),
            b'#' => (tree, yard + 1),
            _ => (tree, yard),
        });
        tree * yard
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (w, h) = self.dim;
        for j in 0..h {
            let slice = &self.data[j * w..(j + 1) * w];
            writeln!(f, "{}", unsafe { str::from_utf8_unchecked(slice) })?;
        }

        Ok(())
    }
}

pub fn puzzle1(input: &str) -> usize {
    let mut grid = Grid::new(input);

    for _ in 0..10 {
        grid.generation();
    }

    grid.count()
}

pub fn puzzle2(input: &str) -> usize {
    const GENERATIONS: usize = 1000000000;

    let mut grid = Grid::new(input);
    let mut values = HashMap::new();
    let mut counts = Vec::new();

    'outer: for i in 0.. {
        counts.push(grid.count());
        if let Some(prev) = values.insert(grid.data.clone(), i) {
            let period = i - prev;
            let result = i - period + ((GENERATIONS - i) % period);

            return counts[result];
        }
        grid.generation();
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"
        .#.#...|#.
        .....#|##|
        .|..|...#.
        ..|#.....#
        #.#|||#|#|
        ...#.||...
        .|....|...
        ||...#|.#|
        |.||||..|.
        ...#.|..|.
    ";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(INPUT), 1147);
    }
}
