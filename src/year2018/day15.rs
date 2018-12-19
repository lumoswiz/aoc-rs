#![allow(bad_style, unused_variables, dead_code)]

use crate::util::Grid;
use nalgebra::Point2;

enum UnitKind {
    Elf,
    Goblin,
}

struct Unit {
    kind: UnitKind,
    pos: Point2<usize>,
}

struct Area {
    grid: Grid,
    units: Vec<Unit>,
}

impl Area {
    fn new(layout: &str) -> Area {
        let mut grid = Grid::from_layout(layout);
        let mut units = Vec::new();

        for (pos, a) in grid.iter_mut() {
            let kind = match *a {
                b'E' => Some(UnitKind::Elf),
                b'G' => Some(UnitKind::Goblin),
                _ => None,
            };
            if let Some(kind) = kind {
                *a = b'.';
                units.push(Unit { kind, pos });
            }
        }

        Area { grid, units }
    }

    fn sort_units(&mut self) {
        self.units.sort_unstable_by_key(|u| (u.pos[1], u.pos[0]));
    }


}

pub fn puzzle1(input: &str) -> i64 {
    0
}

pub fn puzzle2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(""), 0);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }
}
