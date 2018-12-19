use nalgebra::Point2;
use std::fmt::{self, Debug, Formatter};
use std::ops::{Index, IndexMut};
use std::str::{self, FromStr};
use std::usize;

pub fn split<'a>(input: &'a str) -> impl 'a + Iterator<Item = &'a str> {
    input.trim().split('\n').map(|s| s.trim())
}

pub fn parse<'a, T>(input: &'a str) -> impl 'a + Iterator<Item = T>
where
    T: 'a + FromStr,
    T::Err: Debug,
{
    parse_with(input, |s| s.parse::<T>().unwrap())
}

pub fn parse_with<'a, T, F>(input: &'a str, f: F) -> impl 'a + Iterator<Item = T>
where
    T: 'a,
    F: 'a + Fn(&'a str) -> T,
{
    split(input).map(f)
}

#[derive(Clone)]
pub struct Grid {
    size: (usize, usize),
    squares: Vec<u8>,
}

impl Grid {
    pub fn new(w: usize, h: usize) -> Grid {
        let size = (w, h);
        let squares = vec![b'.'; w * h];

        Grid { size, squares }
    }

    pub fn from_layout(layout: &str) -> Grid {
        let (size, squares) = layout.trim().split('\n').map(|l| l.trim()).fold(
            ((0, 0), Vec::with_capacity(layout.len())),
            |((_, h), mut layout), line| {
                layout.extend_from_slice(line.as_bytes());
                ((line.len(), h + 1), layout)
            },
        );

        Grid { size, squares }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.squares
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    fn index_of(&self, pos: Point2<usize>) -> usize {
        let (x, y) = (pos[0], pos[1]);
        let (w, h) = self.size;
        if x < w && y < h {
            x + y * w
        } else {
            usize::MAX
        }
    }

    pub fn get<P: Into<Point2<usize>>>(&self, pos: P) -> Option<u8> {
        let i = self.index_of(pos.into());
        self.squares.get(i).cloned()
    }

    #[allow(dead_code)]
    pub fn get_mut<P: Into<Point2<usize>>>(&mut self, pos: P) -> Option<&mut u8> {
        let i = self.index_of(pos.into());
        self.squares.get_mut(i)
    }

    pub fn iter<'a>(&'a self) -> impl 'a + Iterator<Item = (Point2<usize>, u8)> {
        let (w, _) = self.size;
        self.squares
            .iter()
            .cloned()
            .enumerate()
            .map(move |(i, v)| ([i % w, i / w].into(), v))
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl 'a + Iterator<Item = (Point2<usize>, &'a mut u8)> {
        let (w, _) = self.size;
        self.squares
            .iter_mut()
            .enumerate()
            .map(move |(i, v)| ([i % w, i / w].into(), v))
    }
}

impl<P: Into<Point2<usize>>> Index<P> for Grid {
    type Output = u8;

    fn index<'a>(&'a self, index: P) -> &'a Self::Output {
        let i = self.index_of(index.into());
        self.squares.index(i)
    }
}

impl<P: Into<Point2<usize>>> IndexMut<P> for Grid {
    fn index_mut<'a>(&'a mut self, index: P) -> &'a mut Self::Output {
        let i = self.index_of(index.into());
        self.squares.index_mut(i)
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (w, h) = self.size;
        for j in 0..h {
            let slice = &self.squares[j * w..(j + 1) * w];
            writeln!(f, "{}", unsafe { str::from_utf8_unchecked(slice) })?;
        }

        Ok(())
    }
}
