use crate::util::{self, Grid};
use nalgebra::Point2;
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
}

impl Battle {
    fn new(layout: &str) -> Battle {
        let mut grid = Grid::from_layout(layout);
        let mut units = Vec::new();
        let mut poss = HashMap::new();

        for (pos, a) in grid.iter_mut() {
            if let Some(kind) = UnitKind::from_raw(*a) {
                poss.insert(pos, units.len());
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
            poss,
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

    fn dist(&self, pos: Point2<usize>) -> Option<usize> {
        self.dists.get(&pos).cloned()
    }

    fn outcome(&mut self) -> usize {
        // println!("{:?}", self.grid);

        self.sort_units();

        let mut rounds = 0;
        let mut should_move = true;

        while !self.complete() {
            rounds += 1;

            let mut moved = false;
            let mut died = false;
            for i in 0..self.units.len() {
                let mut unit = self.units[i].clone();
                if unit.health <= 0 {
                    continue;
                }

                if should_move {
                    self.calc_dists(unit.pos);

                    let enemies = self.units.iter().filter(|u| u.kind != unit.kind);
                    let targets = enemies.map(|u| util::adjacent4(u.pos)).flatten();
                    let closest = targets
                        .filter(|p| self.dist(*p).is_some())
                        .min_by_key(|p| (self.dist(*p), p[1], p[0]));

                    if let Some(closest) = closest {
                        if unit.pos != closest {
                            moved = true;
                            self.calc_dists(closest);
                            let next_pos = util::adjacent4(unit.pos)
                                .filter(|p| self.dist(*p).is_some())
                                .min_by_key(|p| (self.dist(*p).unwrap(), p[1], p[0]))
                                .expect("at least one square to move");

                            self.units[i].pos = next_pos;
                            self.grid[unit.pos] = b'.';
                            self.grid[next_pos] = unit.kind.into_raw();
                            self.poss.remove(&unit.pos);
                            self.poss.insert(next_pos, i);
                            unit.pos = next_pos;
                        }
                    }
                }

                let target = util::adjacent4(unit.pos)
                    .filter_map(|p| self.poss.get(&p))
                    .cloned()
                    .filter(|i| self.units[*i].kind != unit.kind)
                    .filter(|i| self.units[*i].health > 0)
                    .min_by_key(|i| {
                        (
                            self.units[*i].health,
                            self.units[*i].pos[1],
                            self.units[*i].pos[0],
                        )
                    });

                if let Some(target) = target {
                    self.units[target].health -= unit.attack;
                    if self.units[target].health <= 0 {
                        died = true;
                        self.grid[self.units[target].pos] = b'.';
                    }
                }
            }

            if died {
                self.units.retain(|u| u.health > 0);
            }
            if died || moved {
                should_move = true;

                self.sort_units();
                self.poss.clear();
                for (i, u) in self.units.iter().enumerate() {
                    self.poss.insert(u.pos, i);
                }
            }

            println!("----- {} -----", rounds);
            // print!("{:?}", self.grid);
            // for u in self.units.iter() {
            //     match u.kind {
            //         UnitKind::Elf => println!("E({}) @{},{}", u.health, u.pos[0], u.pos[1]),
            //         UnitKind::Goblin => println!("G({}) @{},{}", u.health, u.pos[0], u.pos[1]),
            //     }
            // }
            // println!();
        }

        let total_health: i32 = self.units.iter().map(|u| u.health).sum();

        println!("{} {} ", rounds, total_health);
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

        // assert_eq!(
        //     super::puzzle1(
        //         r"
        //             #######
        //             #G..#E#
        //             #E#E.E#
        //             #G.##.#
        //             #...#E#
        //             #...E.#
        //             #######
        //         "
        //     ),
        //     36334
        // );

        // assert_eq!(
        //     super::puzzle1(
        //         r"
        //             #######
        //             #E..EG#
        //             #.#G.E#
        //             #E.##E#
        //             #G..#.#
        //             #..E#.#
        //             #######
        //         "
        //     ),
        //     39514
        // );

        // assert_eq!(
        //     super::puzzle1(
        //         r"
        //             #######
        //             #E.G#.#
        //             #.#G..#
        //             #G.#.G#
        //             #G..#.#
        //             #...E.#
        //             #######
        //         "
        //     ),
        //     27755
        // );

        // assert_eq!(
        //     super::puzzle1(
        //         r"
        //             #######
        //             #.E...#
        //             #.#..G#
        //             #.###.#
        //             #E#G#G#
        //             #...#G#
        //             #######
        //         "
        //     ),
        //     28944
        // );

        // assert_eq!(
        //     super::puzzle1(
        //         r"
        //             #########
        //             #G......#
        //             #.E.#...#
        //             #..##..G#
        //             #...##..#
        //             #...#...#
        //             #.G...G.#
        //             #.....G.#
        //             #########
        //         "
        //     ),
        //     18740
        // );
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }
}
