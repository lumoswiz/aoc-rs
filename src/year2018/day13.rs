use std::fmt::{self, Debug, Formatter};
use std::str;

#[derive(Clone, Copy, Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Debug)]
struct TrainMemory {
    pos: (usize, usize),
    track: u8,
    next_turn: Turn,
}

impl TrainMemory {
    fn new(x: usize, y: usize, dir: u8) -> Option<TrainMemory> {
        let track = match dir {
            b'<' | b'>' => b'-',
            b'^' | b'v' => b'|',
            _ => return None,
        };

        Some(TrainMemory {
            pos: (x, y),
            track,
            next_turn: Turn::Left,
        })
    }
}

struct Railway {
    tracks: Vec<Vec<u8>>,
    trains: Vec<TrainMemory>,
}

impl Railway {
    fn new(input: &str) -> Railway {
        let tracks = input
            .trim_matches('\n')
            .split('\n')
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<_>>();
        let mut trains = Vec::new();
        for y in 0..tracks.len() {
            for x in 0..tracks[y].len() {
                if let Some(train) = TrainMemory::new(x, y, tracks[y][x]) {
                    trains.push(train)
                };
            }
        }

        Railway { tracks, trains }
    }

    fn step(&mut self, collisions: &mut Vec<(usize, usize)>) {
        self.trains.sort_unstable_by_key(|t| (t.pos.1, t.pos.0));
        collisions.clear();

        for i in 0..self.trains.len() {
            if self.trains[i].track == b'X' {
                continue;
            }

            let (x, y) = self.trains[i].pos;
            let dir = self.tracks[y][x];
            let (newx, newy) = match dir {
                b'<' => (x - 1, y),
                b'>' => (x + 1, y),
                b'^' => (x, y - 1),
                b'v' => (x, y + 1),
                _ => {
                    println!("{:?}", self.trains);
                    unreachable!()
                }
            };

            self.tracks[y][x] = self.trains[i].track;
            self.trains[i].track = match self.tracks[newy][newx] {
                b'<' | b'^' | b'>' | b'v' => {
                    let crashed = self
                        .trains
                        .iter_mut()
                        .find(|t| t.pos == (newx, newy))
                        .unwrap();
                    self.tracks[newy][newx] = crashed.track;
                    crashed.track = b'X';

                    collisions.push((newx, newy));

                    b'X'
                }
                track => track,
            };
            self.trains[i].pos = (newx, newy);
            if self.trains[i].track == b'X' {
                continue;
            }

            let turn = match (self.trains[i].track, dir) {
                (b'-', _) | (b'|', _) => Turn::Straight,
                (b'/', b'<') | (b'/', b'>') => Turn::Left,
                (b'/', b'^') | (b'/', b'v') => Turn::Right,
                (b'\\', b'<') | (b'\\', b'>') => Turn::Right,
                (b'\\', b'^') | (b'\\', b'v') => Turn::Left,
                (b'+', _) => self.trains[i].next_turn,
                _ => unreachable!(),
            };
            self.tracks[newy][newx] = match (dir, turn) {
                (b'<', Turn::Left) => b'v',
                (b'<', Turn::Right) => b'^',
                (b'^', Turn::Left) => b'<',
                (b'^', Turn::Right) => b'>',
                (b'>', Turn::Left) => b'^',
                (b'>', Turn::Right) => b'v',
                (b'v', Turn::Left) => b'>',
                (b'v', Turn::Right) => b'<',
                (dir, Turn::Straight) => dir,
                _ => unreachable!(),
            };

            self.trains[i].next_turn = match (self.trains[i].track, self.trains[i].next_turn) {
                (b'+', Turn::Left) => Turn::Straight,
                (b'+', Turn::Straight) => Turn::Right,
                (b'+', Turn::Right) => Turn::Left,
                (_, next_turn) => next_turn,
            }
        }

        if collisions.len() > 0 {
            for i in 0..self.trains.len() {
                while i < self.trains.len() && self.trains[i].track == b'X' {
                    self.trains.swap_remove(i);
                }
            }
        }
    }
}

impl Debug for Railway {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut tracks = String::with_capacity((self.tracks.len() + 1) * self.tracks[0].len());
        for row in self.tracks.iter() {
            tracks.push_str(unsafe { str::from_utf8_unchecked(&row) });
            tracks.push('\n');
        }

        f.debug_struct("Railway")
            .field("tracks", &format_args!("\n{}", tracks))
            .field("trains", &self.trains)
            .finish()
    }
}

pub fn puzzle1(input: &str) -> String {
    let mut railway = Railway::new(input);
    let mut collisions = Vec::new();

    while collisions.is_empty() {
        railway.step(&mut collisions);
    }

    format!("{},{}", collisions[0].0, collisions[0].1)
}

pub fn puzzle2(input: &str) -> String {
    let mut railway = Railway::new(input);
    let mut collisions = Vec::new();

    while railway.trains.len() > 1 {
        railway.step(&mut collisions);
    }

    format!("{},{}", railway.trains[0].pos.0, railway.trains[0].pos.1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        const INPUT: &str = r"
/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";

        assert_eq!(&super::puzzle1(INPUT), "7,3");
    }

    #[test]
    fn puzzle2() {
        const INPUT: &str = r"
/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";

        assert_eq!(super::puzzle2(INPUT), "6,4");
    }
}
