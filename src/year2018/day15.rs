use crate::util::Grid;
use nalgebra::Point2;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::usize;

#[derive(Clone, Copy, Eq, PartialEq)]
enum UnitKind {
    Elf,
    Goblin,
}

impl UnitKind {
    fn from_raw(b: u8) -> Option<UnitKind> {
        match b {
            b'E' => Some(UnitKind::Elf),
            b'G' => Some(UnitKind::Goblin),
            _ => None,
        }
    }

    fn into_raw(&self) -> u8 {
        match self {
            UnitKind::Elf => b'E',
            UnitKind::Goblin => b'G',
        }
    }
}

#[derive(Clone)]
struct Unit {
    kind: UnitKind,
    pos: Point2<usize>,
}

struct Battle {
    grid: Grid,
    units: Vec<Unit>,

    dists: HashMap<Point2<usize>, usize>,
    dists_pending: Vec<Point2<usize>>,
}

impl Battle {
    fn new(layout: &str) -> Battle {
        let mut grid = Grid::from_layout(layout);
        let mut units = Vec::new();

        for (pos, a) in grid.iter_mut() {
            if let Some(kind) = UnitKind::from_raw(*a) {
                units.push(Unit { kind, pos });
            }
        }

        Battle {
            grid,
            units,
            dists: HashMap::new(),
            dists_pending: Vec::new(),
        }
    }

    fn complete(&self) -> bool {
        let mut unit_kinds = self.units.iter().map(|u| u.kind);
        let first = match unit_kinds.next() {
            Some(kind) => kind,
            None => return true,
        };

        unit_kinds.all(|k| k == first)
    }

    fn sort_units(&mut self) {
        self.units.sort_unstable_by_key(|u| (u.pos[1], u.pos[0]));
    }

    fn calc_dists(&mut self, pos: Point2<usize>) {
        self.dists.clear();
        self.dists_pending.clear();

        self.dists.insert(pos, 0);
        self.dists_pending.push(pos);
        while let Some(pos) = self.dists_pending.pop() {
            let dist = self.dists[&pos] + 1;

            for next_pos in adjacent(pos) {
                if self.grid.get(pos).is_none() {
                    continue;
                }

                let next_dist = self.dists.entry(next_pos).or_insert(usize::MAX);
                if dist < *next_dist {
                    *next_dist = dist;
                    self.dists_pending.push(next_pos);
                }
            }
        }
    }

    fn dist(&self, pos: Point2<usize>) -> usize {
        self.dists.get(&pos).cloned().unwrap_or(usize::MAX)
    }

    fn movement(&mut self) -> bool {
        let mut movement = false;
        'units: for i in 0..self.units.len() {
            let unit = self.units[i].clone();
            self.calc_dists(unit.pos);

            let enemies = self.units.iter().filter(|u| u.kind != unit.kind);
            let targets = enemies.map(|u| adjacent(u.pos)).flatten();

            let mut closest = (usize::MAX, 0usize, 0usize);
            for p in targets {
                if p == unit.pos {
                    continue 'units;
                }

                match self.grid.get(p) {
                    Some(b'.') => (),
                    _ => continue,
                };

                closest = cmp::min((self.dist(p), p[1], p[0]), closest);
            }

            if closest.0 == usize::MAX {
                continue;
            }

            movement = true;
            self.calc_dists(Point2::new(closest.2, closest.1));
            let (_, y, x) = adjacent(unit.pos)
                .map(|p| (p, self.dist(p)))
                .fold((usize::MAX, 0usize, 0usize), |s, (p, dist)| {
                    cmp::min((dist, p[1], p[0]), s)
                });

            let next_pos = Point2::new(x, y);
            self.units[i].pos = next_pos;
            self.grid[unit.pos] = b'.';
            self.grid[next_pos] = unit.kind.into_raw();

            println!("MOVE:\n{:?}", self.grid);
        }

        movement
    }
    
    fn attack(&mut self, moved: bool) {
        for i in 0..self.units.len() {
            let kind = self.units[i].kind;

        }
    }

    fn outcome(&mut self) {
        // while !self.complete() {
        for _ in 0..5 {
            let moved = self.movement();
            self.attack(moved);
        }
    }
}

fn adjacent(pos: Point2<usize>) -> impl Iterator<Item = Point2<usize>> {
    static ADJACENT: [(isize, isize); 4] = [(0, 1), (-1, 0), (1, 0), (0, -1)];
    ADJACENT
        .iter()
        .map(move |(dx, dy)| ((pos[0] as isize) + dx, (pos[1] as isize) + dy))
        .map(|(x, y)| Point2::new(x as usize, y as usize))
}

pub fn puzzle1(input: &str) -> i64 {
    let mut battle = Battle::new(input);
    battle.outcome();

    0
}

pub fn puzzle2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        assert_eq!(
            super::puzzle1(
                r"
                    #######
                    #.G...#
                    #...EG#
                    #.#.#G#
                    #..G#E#
                    #.....#
                    #######
                "
            ),
            47
        );
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }
}
