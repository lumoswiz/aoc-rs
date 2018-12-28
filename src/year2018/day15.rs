use crate::util::{self, Grid};
use nalgebra::Point2;
use std::cmp;
use std::collections::HashMap;
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
    attack: i32,
    health: i32,
}

struct Battle {
    grid: Grid,
    units: Vec<Unit>,

    dists: HashMap<Point2<usize>, usize>,
    dists_pending: Vec<Point2<usize>>,
    poss: HashMap<Point2<usize>, usize>,
    targets: Vec<(usize, usize)>,
}

impl Battle {
    fn new(layout: &str) -> Battle {
        let mut grid = Grid::from_layout(layout);
        let mut units = Vec::new();

        for (pos, a) in grid.iter_mut() {
            if let Some(kind) = UnitKind::from_raw(*a) {
                units.push(Unit {
                    kind,
                    pos,
                    attack: 3,
                    health: 200,
                });
            }
        }

        Battle {
            grid,
            units,
            dists: HashMap::new(),
            dists_pending: Vec::new(),
            poss: HashMap::new(),
            targets: Vec::new(),
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

            for next_pos in util::adjacent4(pos) {
                if self.grid.get(next_pos) != Some(b'.') {
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

            // let mut y = 0;
            // for (p, _) in self.grid.iter() {
            //     if p[1] != y {
            //         println!();
            //         y = p[1];
            //     }
            //     match self.dists.get(&p) {
            //         Some(d) if *d != usize::MAX => print!("{}", d),
            //         _ => print!("#"),
            //     };
            // }
            // println!();

            let enemies = self.units.iter().filter(|u| u.kind != unit.kind);
            let targets = enemies.map(|u| util::adjacent4(u.pos)).flatten();

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
            let (_, y, x) = util::adjacent4(unit.pos)
                .map(|p| (p, self.dist(p)))
                .fold((usize::MAX, 0usize, 0usize), |s, (p, dist)| {
                    cmp::min((dist, p[1], p[0]), s)
                });

            let next_pos = Point2::new(x, y);
            self.units[i].pos = next_pos;
            self.grid[unit.pos] = b'.';
            self.grid[next_pos] = unit.kind.into_raw();
        }

        if movement {
            println!("MOVE:\n{:?}", self.grid);
        }

        movement
    }

    fn store_poss(&mut self) {
        self.poss.clear();
        for (i, unit) in self.units.iter().enumerate() {
            self.poss.insert(unit.pos, i);
        }
    }

    fn attack(&mut self, moved: bool) -> usize {
        self.store_poss();

        self.targets.clear();
        for i in 0..self.units.len() {
            let kind = self.units[i].kind;
            let target = util::adjacent4(self.units[i].pos)
                .filter_map(|p| self.poss.get(&p))
                .filter(|i| self.units[**i].kind != kind)
                .min_by_key(|i| {
                    (
                        self.units[**i].health,
                        self.units[**i].pos[1],
                        self.units[**i].pos[0],
                    )
                });

            if let Some(target) = target {
                self.targets.push((*target, i));
            }
        }

        println!("{:?}", self.targets);
        let mut rounds = 0;
        loop {
            rounds += 1;
            let mut died = false;
            for (target, attacker) in self.targets.iter().cloned() {
                if self.units[attacker].health <= 0 {
                    continue;
                }
                self.units[target].health -= self.units[attacker].attack;
                if self.units[target].health <= 0 {
                    died = true;
                    self.grid[self.units[target].pos] = b'.';
                }
            }

            if died {
                self.units.retain(|u| u.health > 0);
                println!("ATTACK:\n{:?}", self.grid);
            }

            if moved || died {
                break;
            }
        }

        for u in self.units.iter() {
            match u.kind {
                UnitKind::Elf => println!("E({}) @{},{}", u.health, u.pos[0], u.pos[1]),
                UnitKind::Goblin => println!("G({}) @{},{}", u.health, u.pos[0], u.pos[1]),
            }
        }

        rounds
    }

    fn outcome(&mut self) -> usize {
        // for _ in 0..9 {
        let mut rounds = 0;
        while !self.complete() {
            self.sort_units();
            let moved = self.movement();
            rounds += self.attack(moved);

            println!("----- {} -----", rounds);
        }

        let total_health: i32 = self.units.iter().map(|u| u.health).sum();
        rounds * (total_health as usize)
    }
}

pub fn puzzle1(input: &str) -> usize {
    let mut battle = Battle::new(input);
    battle.outcome()
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
            27730
        );

        assert_eq!(
            super::puzzle1(
                r"
                    #######
                    #G..#E#
                    #E#E.E#
                    #G.##.#
                    #...#E#
                    #...E.#
                    #######
                "
            ),
            36334
        );

        assert_eq!(
            super::puzzle1(
                r"
                    #######
                    #E..EG#
                    #.#G.E#
                    #E.##E#
                    #G..#.#
                    #..E#.#
                    #######
                "
            ),
            39514
        );

        assert_eq!(
            super::puzzle1(
                r"
                    #######
                    #E.G#.#
                    #.#G..#
                    #G.#.G#
                    #G..#.#
                    #...E.#
                    #######
                "
            ),
            27755
        );

        assert_eq!(
            super::puzzle1(
                r"
                    #######
                    #.E...#
                    #.#..G#
                    #.###.#
                    #E#G#G#
                    #...#G#
                    #######
                "
            ),
            28944
        );

        assert_eq!(
            super::puzzle1(
                r"
                    #########
                    #G......#
                    #.E.#...#
                    #..##..G#
                    #...##..#
                    #...#...#
                    #.G...G.#
                    #.....G.#
                    #########
                "
            ),
            18740
        );
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }
}
