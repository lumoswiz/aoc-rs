use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
struct Check {
    before: [i32; 4],
    opcode: i32,
    params: (i32, i32, i32),
    after: [i32; 4],
}

lazy_static! {
    static ref CHECK_PATTERN: Regex =
        Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]\n(\d+) (\d+) (\d+) (\d+)\nAfter:  \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
}

impl FromStr for Check {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = CHECK_PATTERN
            .captures(s)
            .ok_or_else(|| failure::err_msg("does not match check pattern"))?;

        Ok(Check {
            before: [c[1].parse()?, c[2].parse()?, c[3].parse()?, c[4].parse()?],
            opcode: c[5].parse()?,
            params: (c[6].parse()?, c[7].parse()?, c[8].parse()?),
            after: [
                c[9].parse()?,
                c[10].parse()?,
                c[11].parse()?,
                c[12].parse()?,
            ],
        })
    }
}

#[derive(Debug)]
struct Inst {
    opcode: i32,
    params: (i32, i32, i32),
}

lazy_static! {
    static ref INST_PATTERN: Regex = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
}

impl FromStr for Inst {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = INST_PATTERN
            .captures(s)
            .ok_or_else(|| failure::err_msg("does not match inst pattern"))?;

        Ok(Inst {
            opcode: c[1].parse()?,
            params: (c[2].parse()?, c[3].parse()?, c[4].parse()?),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Op {
    fn values() -> impl Iterator<Item = Op> {
        static VALUES: [Op; 16] = [
            Op::Addr,
            Op::Addi,
            Op::Mulr,
            Op::Muli,
            Op::Banr,
            Op::Bani,
            Op::Borr,
            Op::Bori,
            Op::Setr,
            Op::Seti,
            Op::Gtir,
            Op::Gtri,
            Op::Gtrr,
            Op::Eqir,
            Op::Eqri,
            Op::Eqrr,
        ];
        VALUES.iter().cloned()
    }

    #[rustfmt::skip]
    pub fn exec(&self, (a, b, c): (i32, i32, i32), registers: &mut [i32]) {
        macro_rules! r {
            ($i:expr) => { registers[$i as usize] };
        }

        r!(c) = match self {
            Op::Addr => r!(a) + r!(b),
            Op::Addi => r!(a) + b,
            Op::Mulr => r!(a) * r!(b),
            Op::Muli => r!(a) * b,
            Op::Banr => r!(a) & r!(b),
            Op::Bani => r!(a) & b,
            Op::Borr => r!(a) | r!(b),
            Op::Bori => r!(a) | b,
            Op::Setr => r!(a),
            Op::Seti => a,
            Op::Gtir => if a > r!(b) { 1 } else { 0 },
            Op::Gtri => if r!(a) > b { 1 } else { 0 },
            Op::Gtrr => if r!(a) > r!(b) { 1 } else { 0 },
            Op::Eqir => if a == r!(b) { 1 } else { 0 },
            Op::Eqri => if r!(a) == b { 1 } else { 0 },
            Op::Eqrr => if r!(a) == r!(b) { 1 } else { 0 },
        };
    }
}

pub fn puzzle1(input: &str) -> usize {
    let input = input
        .trim()
        .split("\n\n\n\n")
        .nth(0)
        .expect("missing checks");
    let checks = input
        .split("\n\n")
        .map(|s| s.parse::<Check>().expect("check parse error"));

    let mut count = 0;
    for check in checks {
        let mut matches = 0;
        for op in Op::values() {
            let params = check.params.clone();
            let mut registers = check.before.clone();
            op.exec(params, &mut registers);
            if registers == check.after {
                matches += 1;
            }
        }
        if matches >= 3 {
            count += 1;
        }
    }

    count
}

pub fn puzzle2(input: &str) -> i32 {
    let mut input = input.trim().split("\n\n\n\n");

    let checks = input
        .next()
        .expect("missing checks")
        .split("\n\n")
        .map(|s| s.parse::<Check>().expect("check parse error"));
    let insts = input
        .next()
        .expect("missing insts")
        .trim()
        .split('\n')
        .map(|s| s.parse::<Inst>().expect("inst parse error"));

    let mut check_result: Vec<HashSet<Op>> = vec![Op::values().collect(); 16];
    for check in checks {
        for op in Op::values() {
            let params = check.params.clone();
            let mut registers = check.before.clone();
            op.exec(params, &mut registers);
            if registers != check.after {
                check_result[check.opcode as usize].remove(&op);
            }
        }
    }

    let mut lookup = [Op::Addr; 16];
    'outer: for _ in 0..16 {
        for i in 0..16 {
            if check_result[i].len() == 1 {
                let op = *check_result[i].iter().next().unwrap();
                lookup[i] = op;
                for j in 0..16 {
                    check_result[j].remove(&op);
                }
                continue 'outer;
            }
        }
        panic!("{:?}", check_result);
    }

    let mut registers = [0; 4];
    for inst in insts {
        lookup[inst.opcode as usize].exec(inst.params, &mut registers);
    }

    registers[0]
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        const INPUT: &str = "Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]";
        assert_eq!(super::puzzle1(INPUT), 1);
    }
}
