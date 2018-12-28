use crate::util::{self, Grid};
use nalgebra::Point2;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet, VecDeque};
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

    fn enemy(&self) -> UnitKind {
        match self {
            UnitKind::Elf => UnitKind::Goblin,
            UnitKind::Goblin => UnitKind::Elf,
        }
    }
}

struct Unit {
    kind: UnitKind,
    pos: Point2<usize>,
    attack: i32,
    health: i32,
}

struct Pathfinder {
    seen: HashSet<Point2<usize>>,
    targets: HashSet<Point2<usize>>,
    pending: VecDeque<(Point2<usize>, Point2<usize>, usize)>,
}

impl Pathfinder {
    fn new() -> Pathfinder {
        Pathfinder {
            seen: HashSet::new(),
            targets: HashSet::new(),
            pending: VecDeque::new(),
        }
    }

    fn search<I: Iterator<Item = Point2<usize>>>(
        &mut self,
        start: Point2<usize>,
        targets: I,
        grid: &Grid,
    ) -> Option<Point2<usize>> {
        self.seen.clear();
        self.targets.clear();
        self.pending.clear();

        self.seen.insert(start);
        for target in targets {
            if target == start {
                return None;
            }
            self.targets.insert(target);
        }
        for p in util::adjacent4(start).filter(|p| grid.get(*p) == Some(b'.')) {
            self.pending.push_back((p, p, 1));
        }

        let mut best: Option<(Point2<usize>, Point2<usize>, usize)> = None;
        while let Some((start, next, dist)) = self.pending.pop_front() {
            if best.map(|b| b.2).unwrap_or(dist) < dist {
                break;
            }

            if self.targets.contains(&next) {
                best = match best {
                    Some((s, n, d))
                        if (d, n[1], n[0], s[1], s[0])
                            < (dist, next[1], next[0], start[1], start[0]) =>
                    {
                        Some((s, n, d))
                    }
                    _ => Some((start, next, dist)),
                }
            } else {
                for p in util::adjacent4(next).filter(|p| grid.get(*p) == Some(b'.')) {
                    if self.seen.insert(p) {
                        self.pending.push_back((start, p, dist + 1));
                    }
                }
            }
        }

        best.map(|b| b.0)
    }
}

struct Battle {
    grid: Grid,
    units: Vec<Unit>,
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

        Battle { grid, units }
    }

    fn outcome(&mut self) -> i32 {
        println!("{:?}", self.grid);

        self.sort_units();

        let mut pathfinder = Pathfinder::new();
        let mut rounds = 0;
        let mut should_move = true;

        //'battle: while !self.complete() {
        'battle: for _ in 0..5 {
            let mut updated = false;

            let mut i = 0;
            while i < self.units.len() {
                macro_rules! u {
                    () => {
                        self.units[i]
                    };
                }

                let mut enemies = self.units.iter().filter(|u| u!().kind != u.kind).peekable();
                if let None = enemies.peek() {
                    break 'battle;
                }

                if should_move {
                    let targets = enemies
                        .map(|u| util::adjacent4(u.pos))
                        .flatten()
                        .filter(|p| *p == u!().pos || self.grid.get(*p) == Some(b'.'));
                    let move_to = pathfinder.search(u!().pos, targets, &self.grid);
                    if let Some(move_to) = move_to {
                        updated = true;
                        self.grid[u!().pos] = b'.';
                        self.grid[move_to] = u!().kind.into_raw();
                        u!().pos = move_to;
                    }
                }

                let enemy = util::adjacent4(u!().pos)
                    .filter(|p| self.grid.get(*p) == Some(u!().kind.enemy().into_raw()))
                    .map(|p| {
                        self.units
                            .iter()
                            .enumerate()
                            .find(|(_, u)| u.pos == p)
                            .expect("unit on grid but not in registry")
                    })
                    .min_by_key(|(_, u)| (u.health, u.pos[1], u.pos[0]))
                    .map(|(i, _)| i);
                if let Some(enemy) = enemy {
                    println!("{} attacks {}", i, enemy);
                }

                i += 1;
            }

            if updated {}

            should_move = updated;
            rounds += 1;

            println!("----- {} -----", rounds);
            println!("{:?}", self.grid);
        }

        0
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
}

/*
impl Battle {


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
*/

pub fn puzzle1(input: &str) -> i32 {
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
