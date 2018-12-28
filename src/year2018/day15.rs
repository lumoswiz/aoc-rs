use crate::util::{self, Grid};
use nalgebra::Point2;
use std::collections::hash_map::Entry;
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

#[derive(Clone)]
struct Unit {
    kind: UnitKind,
    pos: Point2<usize>,
    attack: i32,
    health: i32,
}

struct Pathfinder {
    seen: HashMap<Point2<usize>, (Point2<usize>, usize)>,
    targets: HashSet<Point2<usize>>,
    pending: VecDeque<Point2<usize>>,
}

impl Pathfinder {
    fn new() -> Pathfinder {
        Pathfinder {
            seen: HashMap::new(),
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

        self.seen.insert(start, (start, 0));
        for target in targets {
            if target == start {
                return None;
            }
            self.targets.insert(target);
        }

        if self.targets.is_empty() {
            return None;
        }

        for p in util::adjacent4(start).filter(|p| grid.get(*p) == Some(b'.')) {
            self.pending.push_back(p);
            self.seen.insert(p, (p, 1));
        }

        let mut best: Option<(Point2<usize>, Point2<usize>, usize)> = None;
        while let Some(next) = self.pending.pop_front() {
            let (start, dist) = self.seen[&next];
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
                let dist = dist + 1;
                for p in util::adjacent4(next).filter(|p| grid.get(*p) == Some(b'.')) {
                    match self.seen.entry(p) {
                        Entry::Occupied(mut o) => {
                            let o = o.get_mut();
                            if (o.1, o.0[1], o.0[0]) > (dist, start[1], start[0]) {
                                *o = (start, dist);
                            }
                        }
                        Entry::Vacant(e) => {
                            e.insert((start, dist));
                            self.pending.push_back(p);
                        }
                    }
                }
            }
        }

        best.map(|b| b.0)
    }
}

#[derive(Clone)]
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
        // println!("{:?}", self.grid);

        self.sort_units();

        let mut pathfinder = Pathfinder::new();
        let mut rounds = 0;
        let mut should_move = true;

        'battle: while !self.complete() {
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
                    self.units[enemy].health -= u!().attack;
                    if self.units[enemy].health <= 0 {
                        updated = true;
                        self.grid[self.units[enemy].pos] = b'.';
                        self.units.remove(enemy);
                        if enemy < i {
                            i -= 1
                        }
                    }
                }

                should_move |= updated;
                i += 1;
            }

            if updated {
                self.sort_units();
            }

            should_move = updated;
            rounds += 1;

            // println!("----- {} -----", rounds);
            // print!("{:?}", self.grid);
            // for u in self.units.iter() {
            //     match u.kind {
            //         UnitKind::Elf => println!("E({}) @{},{}", u.health, u.pos[0], u.pos[1]),
            //         UnitKind::Goblin => println!("G({}) @{},{}", u.health, u.pos[0], u.pos[1]),
            //     }
            // }
            // println!();
        }

        rounds * self.units.iter().map(|u| u.health).sum::<i32>()
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

    fn elves_count(&self) -> usize {
        self.units
            .iter()
            .filter(|u| u.kind == UnitKind::Elf)
            .count()
    }
}

pub fn puzzle1(input: &str) -> i32 {
    let mut battle = Battle::new(input);
    battle.outcome()
}

pub fn puzzle2(input: &str) -> i32 {
    let battle = Battle::new(input);
    let initial_elves_count = battle.elves_count();

    for attack in 4.. {
        let mut battle = battle.clone();
        for u in battle.units.iter_mut().filter(|u| u.kind == UnitKind::Elf) {
            u.attack = attack;
        }

        let outcome = battle.outcome();
        if battle.elves_count() == initial_elves_count {
            return outcome;
        }
    }

    unreachable!();
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
        assert_eq!(
            super::puzzle2(
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
            4988
        );

        assert_eq!(
            super::puzzle2(
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
            31284
        );

        assert_eq!(
            super::puzzle2(
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
            3478
        );

        assert_eq!(
            super::puzzle2(
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
            6474
        );

        assert_eq!(
            super::puzzle2(
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
            1140
        );
    }
}
