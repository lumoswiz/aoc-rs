use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::str::FromStr;

struct Game(usize, usize);

lazy_static! {
    static ref GAME_PATTERN: Regex =
        Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = GAME_PATTERN
            .captures(s)
            .ok_or_else(|| failure::err_msg("does not match game pattern"))?;

        Ok(Game(c[1].parse()?, c[2].parse()?))
    }
}

struct Marbles {
    l: VecDeque<usize>,
}

impl Marbles {
    fn new(nmarbles: usize) -> Marbles {
        let mut l = VecDeque::with_capacity(nmarbles + 1);
        l.push_back(0);

        Marbles { l }
    }

    fn rotate(&mut self, offset: isize) {
        if self.l.is_empty() {
            return;
        }

        let clockwise = offset > 0;
        for _ in 0..offset.abs() {
            if clockwise {
                let current = self.l.pop_front().unwrap();
                self.l.push_back(current);
            } else {
                let previous = self.l.pop_back().unwrap();
                self.l.push_front(previous);
            }
        }
    }

    fn push(&mut self, value: usize) {
        self.rotate(2);
        self.l.push_front(value);
    }

    fn score(&mut self) -> usize {
        self.rotate(-7);
        self.l.pop_front().unwrap_or(0)
    }
}

fn puzzle(input: &str, factor: usize) -> usize {
    let Game(nplayers, nmarbles) = input.parse().unwrap();
    let nmarbles = nmarbles * factor;

    let mut scores = vec![0; nplayers];
    let mut marbles = Marbles::new(nmarbles);

    for marble in 1..=nmarbles {
        let player = (marble - 1) % nplayers;

        match marble % 23 {
            0 => scores[player] += marble + marbles.score(),
            _ => marbles.push(marble),
        }
    }

    scores.iter().cloned().max().unwrap_or(0)
}

pub fn puzzle1(input: &str) -> usize {
    puzzle(input, 1)
}

pub fn puzzle2(input: &str) -> usize {
    puzzle(input, 100)
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        assert_eq!(
            super::puzzle1("9 players; last marble is worth 25 points"),
            32
        );
        assert_eq!(
            super::puzzle1("10 players; last marble is worth 1618 points"),
            8317
        );
        assert_eq!(
            super::puzzle1("13 players; last marble is worth 7999 points"),
            146373
        );
        assert_eq!(
            super::puzzle1("17 players; last marble is worth 1104 points"),
            2764
        );
        assert_eq!(
            super::puzzle1("21 players; last marble is worth 6111 points"),
            54718
        );
        assert_eq!(
            super::puzzle1("30 players; last marble is worth 5807 points"),
            37305
        );
    }
}
