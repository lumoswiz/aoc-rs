use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Position(pub i32, pub i32);

#[derive(Debug, Copy, Clone)]
pub enum Facing {
    North,
    South,
    East,
    West,
}

impl Default for Facing {
    fn default() -> Self {
        Facing::North
    }
}

impl Facing {
    pub fn delta(&self) -> (i32, i32) {
        match *self {
            Facing::North => (0, 1),
            Facing::South => (0, -1),
            Facing::East => (1, 0),
            Facing::West => (-1, 0),
        }
    }
}

#[derive(Debug)]
pub struct Santa {
    facing: Facing,
    position: Position,
}

impl Default for Santa {
    fn default() -> Self {
        Self {
            facing: Facing::default(),
            position: Position::default(),
        }
    }
}

impl Santa {
    pub fn turn(&mut self, t: Turn) {
        self.facing = match (t, &self.facing) {
            (Turn::Left, Facing::North) => Facing::West,
            (Turn::Left, Facing::West) => Facing::South,
            (Turn::Left, Facing::South) => Facing::East,
            (Turn::Left, Facing::East) => Facing::North,
            (Turn::Right, Facing::North) => Facing::East,
            (Turn::Right, Facing::East) => Facing::South,
            (Turn::Right, Facing::South) => Facing::West,
            (Turn::Right, Facing::West) => Facing::North,
        }
    }

    pub fn step(&mut self, n: u16) {
        let s = i32::from(n);
        let (dx, dy) = self.facing.delta();
        let x = self.position.0;
        let y = self.position.1;
        self.position = Position(x + dx * s, y + dy * s);
    }

    pub fn walk(&mut self, instruction: Instruction) {
        self.turn(instruction.turn);
        self.step(instruction.steps);
    }

    pub fn distance(&self) -> u32 {
        (self.position.0.abs() + self.position.1.abs()) as u32
    }
}

#[derive(Debug)]
pub enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Instruction {
    turn: Turn,
    steps: u16,
}

fn parse_instruction(token: &str) -> Result<Instruction, &'static str> {
    let mut item = token.trim().chars();
    let turn = match item.next() {
        Some('L') => Turn::Left,
        Some('R') => Turn::Right,
        _ => return Err("expected 'L' or 'R'"),
    };
    let steps: u16 = match item.as_str().parse() {
        Ok(n) => n,
        Err(_) => return Err("bad number"),
    };
    Ok(Instruction { turn, steps })
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>, &'static str> {
    input
        .split(',')
        .map(|s| s.trim())
        .filter(|&s| !s.is_empty())
        .map(|token| parse_instruction(token))
        .collect()
}

pub fn puzzle1(input: &str) -> u32 {
    let mut santa = Santa::default();
    match parse_instructions(input) {
        Ok(instructions) => {
            for instr in instructions {
                santa.walk(instr);
            }
            return santa.distance();
        }
        Err(e) => {
            eprint!("{:?}", e);
            0
        }
    }
}

pub fn puzzle2(input: &str) -> u32 {
    let mut santa = Santa::default();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    let instructions = match parse_instructions(input) {
        Ok(v) => v,
        Err(e) => {
            eprint!("{:?}", e);
            return 0;
        }
    };

    for instruction in instructions {
        santa.turn(instruction.turn);
        for _ in 0..instruction.steps {
            santa.step(1u16);
            let pos = (santa.position.0, santa.position.1);
            if !visited.insert(pos) {
                return santa.distance();
            }
        }
    }
    santa.distance()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "R8, R4, R4, R8";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 8);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 4);
    }
}
