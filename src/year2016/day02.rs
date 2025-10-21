pub enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default)]
pub struct Position(pub i8, pub i8);

impl Position {
    pub fn get_digit(&self) -> u32 {
        match (self.0, self.1) {
            (-1, 1) => 1,
            (0, 1) => 2,
            (1, 1) => 3,
            (-1, 0) => 4,
            (0, 0) => 5,
            (1, 0) => 6,
            (-1, -1) => 7,
            (0, -1) => 8,
            (1, -1) => 9,
            _ => unreachable!("out of bounds"),
        }
    }

    pub fn get_key(&self) -> char {
        match (self.0, self.1) {
            (0, 2) => '1',
            (-1, 1) => '2',
            (0, 1) => '3',
            (1, 1) => '4',
            (-2, 0) => '5',
            (-1, 0) => '6',
            (0, 0) => '7',
            (1, 0) => '8',
            (2, 0) => '9',
            (-1, -1) => 'A',
            (0, -1) => 'B',
            (1, -1) => 'C',
            (0, -2) => 'D',
            _ => unreachable!("out of bounds by construction"),
        }
    }

    pub fn step_with<F>(&mut self, instruction: Instruction, in_bounds: F) -> &mut Self
    where
        F: Fn(i8, i8) -> bool + Copy,
    {
        let (dx, dy) = match instruction {
            Instruction::Up => (0, 1),
            Instruction::Down => (0, -1),
            Instruction::Right => (1, 0),
            Instruction::Left => (-1, 0),
        };
        let (nx, ny) = (self.0 + dx, self.1 + dy);
        if in_bounds(nx, ny) {
            self.0 = nx;
            self.1 = ny;
        }
        self
    }

    pub fn step_all_with<I, F>(&mut self, instructions: I, in_bounds: F) -> &mut Self
    where
        I: IntoIterator<Item = Instruction>,
        F: Fn(i8, i8) -> bool + Copy,
    {
        for instruction in instructions {
            self.step_with(instruction, in_bounds);
        }
        self
    }
}

fn square(n: i8) -> impl Fn(i8, i8) -> bool + Copy {
    move |x, y| (-n..=n).contains(&x) && (-n..=n).contains(&y)
}

fn diamond(n: i8) -> impl Fn(i8, i8) -> bool + Copy {
    move |x, y| (-n..=n).contains(&x) && (-n..=n).contains(&y) && (x.abs() + y.abs() <= n)
}

fn parse_input(input: &str) -> Vec<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            line.trim_end_matches('\r')
                .chars()
                .map(|c| match c {
                    'U' => Instruction::Up,
                    'D' => Instruction::Down,
                    'L' => Instruction::Left,
                    'R' => Instruction::Right,
                    _ => panic!("unexepcted char"),
                })
                .collect::<Vec<Instruction>>()
        })
        .collect()
}

pub fn puzzle1(input: &str) -> u32 {
    let mut pos = Position::default();
    parse_input(input).into_iter().fold(0u32, |acc, row| {
        pos.step_all_with(row, square(1));
        acc * 10 + pos.get_digit()
    })
}

pub fn puzzle2(input: &str) -> String {
    let mut pos = Position(-2, 0);
    parse_input(input)
        .into_iter()
        .map(|row| {
            pos.step_all_with(row, diamond(2));
            pos.get_key()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "ULL
RRDDD
LURDL
UUUUD";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 1985);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), "5DB3");
    }
}
