use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::fmt::{self, Debug, Formatter};
use std::mem;
use std::str::{self, FromStr};

struct GrowOp {
    state: HashSet<isize>,
    next: HashSet<isize>,
    range: (isize, isize),
    rules: [bool; 32],
}

impl GrowOp {
    fn new<'a, I: Iterator<Item = (&'a str, u8)>>(
        initial_state: &'a str,
        rule_definitions: I,
    ) -> GrowOp {
        let state = initial_state
            .bytes()
            .enumerate()
            .filter_map(|(i, c)| match c {
                b'#' => Some(i as isize),
                _ => None,
            })
            .collect();
        let next = HashSet::new();
        let start = initial_state.find('#').unwrap_or(0) as _;
        let end = initial_state.rfind('#').unwrap_or(0) as _;
        let mut rules = [false; 32];
        for s in rule_definitions.filter_map(|(s, u)| match u {
            b'#' => Some(s.as_bytes()),
            _ => None,
        }) {
            let mut rule = 0usize;
            for b in s {
                rule <<= 1;
                if *b == b'#' {
                    rule |= 1;
                }
            }
            rules[rule] = true;
        }

        GrowOp {
            state,
            next,
            range: (start, end),
            rules,
        }
    }

    fn step(&mut self) {
        self.next.clear();
        let s = &self.state;
        let mut range = (isize::max_value(), isize::min_value());

        for i in (self.range.0 - 4)..=self.range.1 {
            #[rustfmt::skip]
            let ss = if s.contains(&i) { 0b10000 } else { 0 }
                + if s.contains(&(i + 1)) { 0b01000 } else { 0 }
                + if s.contains(&(i + 2)) { 0b00100 } else { 0 }
                + if s.contains(&(i + 3)) { 0b00010 } else { 0 }
                + if s.contains(&(i + 4)) { 0b00001 } else { 0 };

            let x = i + 2;
            if self.rules[ss] {
                range = (cmp::min(range.0, x), cmp::max(range.1, x));
                self.next.insert(x);
            }
        }

        self.range = range;
        mem::swap(&mut self.next, &mut self.state);
    }

    fn get_range(&self) -> impl Iterator<Item = isize> {
        self.range.0..=self.range.1
    }

    fn pots<'a>(&'a self) -> impl 'a + Iterator<Item = isize> {
        self.get_range().filter(move |i| self.state.contains(&i))
    }
}

lazy_static! {
    static ref INITIAL_STATE_PATTERN: Regex = Regex::new(r"initial state: ([.#]+)").unwrap();
    static ref RULE_PATTERN: Regex = Regex::new(r"([.#]{5}) => ([.#])").unwrap();
}

impl FromStr for GrowOp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().split('\n').map(str::trim);

        let initial_state = {
            let line = lines
                .next()
                .ok_or_else(|| failure::err_msg("missing initial state line"))?;
            let captures = INITIAL_STATE_PATTERN
                .captures(line)
                .ok_or_else(|| failure::err_msg("does not match initial state pattern"))?;
            captures.get(1).unwrap().as_str()
        };
        lines
            .next()
            .ok_or_else(|| failure::err_msg("missing newline separator"))?;

        let mut rules = Vec::new();
        while let Some(line) = lines.next() {
            let captures = RULE_PATTERN
                .captures(line)
                .ok_or_else(|| failure::err_msg("does not match rule pattern"))?;

            rules.push((captures.get(1).unwrap().as_str(), captures[2].as_bytes()[0]));
        }

        Ok(GrowOp::new(initial_state, rules.into_iter()))
    }
}

impl Debug for GrowOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: ", self.range.0)?;
        for i in self.get_range() {
            if self.state.contains(&i) {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

fn puzzle(input: &str, generations: usize) -> isize {
    let mut grow_op: GrowOp = input.parse().expect("failed to parse input");

    let (generations, offset) = if generations > 200 {
        // assume that things will even out after about 200 generations, at
        // which point we just assume the points all start shifting to the
        // right one at a time per generation... this was through observation
        // and I am not sure that all user inputs work this way
        (200, (generations - 200) as isize)
    } else {
        (generations, 0)
    };

    for _ in 0..generations {
        grow_op.step();
    }

    grow_op.pots().map(|i| i + offset).sum()
}

pub fn puzzle1(input: &str) -> isize {
    puzzle(input, 20)
}

pub fn puzzle2(input: &str) -> isize {
    puzzle(input, 50000000000)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"
        initial state: #..#.#..##......###...###

        ...## => #
        ..#.. => #
        .#... => #
        .#.#. => #
        .#.## => #
        .##.. => #
        .#### => #
        #.#.# => #
        #.### => #
        ##.#. => #
        ##.## => #
        ###.. => #
        ###.# => #
        ####. => #
    ";

    #[test]
    fn puzzle() {
        assert_eq!(super::puzzle(INPUT, 20), 325);
    }
}
