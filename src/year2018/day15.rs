/*
use crate::util::Grid;
use nalgebra::Point2;

#[derive(Clone, Copy, Eq, PartialEq)]
enum UnitKind {
    Elf,
    Goblin,
}

struct Unit {
    kind: UnitKind,
    pos: Point2<usize>,
}

struct Battle {
    grid: Grid,
    units: Vec<Unit>,
}

impl Battle {
    fn new(layout: &str) -> Battle {
        let mut grid = Grid::from_layout(layout);
        let mut units = Vec::new();

        for (pos, a) in grid.iter_mut() {
            let kind = match *a {
                b'E' => Some(UnitKind::Elf),
                b'G' => Some(UnitKind::Goblin),
                _ => None,
            };
            if let Some(kind) = kind {
                units.push(Unit { kind, pos });
            }
        }

        Battle { grid, units }
    }

    fn complete(&self) -> bool {
        let mut unit_kinds = self.units.iter().map(|u| u.kind);
        let first_kind = match unit_kinds.next() {
            Some(kind) => kind,
            None => return true,
        };

        unit_kinds.all(|k| k == first_kind)
    }

    fn sort_units(&mut self) {
        self.units.sort_unstable_by_key(|u| (u.pos[1], u.pos[0]));
    }

    fn movement(&mut self) {
        for unit in self.units.iter() {
            let enemies = self
                .units
                .iter()
                .filter(|u| u.kind != unit.kind);
            let targets = enemies
                .map(|u| adjacent(u.pos))
                .flatten()
                .filter(|p| *p == unit.pos || self.grid.get(*p) == Some(b'.'));
        }
    }

    fn battle(&mut self) {
        
    }
}
*/

pub fn puzzle1(_input: &str) -> i64 {
    0
}

pub fn puzzle2(_input: &str) -> i64 {
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
