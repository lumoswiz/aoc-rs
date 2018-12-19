use crate::year2018::day16::Op;
use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct Inst(Op, (i32, i32, i32));

impl Inst {
    fn exec(&self, registers: &mut [i32]) {
        self.0.exec(self.1, registers);
    }
}

lazy_static! {
    static ref IP_PATTERN: Regex = Regex::new(r"#ip (\d)").unwrap();
    static ref INST_PATTERN: Regex = Regex::new(r"([a-z]{4}) (\d+) (\d+) (\d+)").unwrap();
}

impl FromStr for Inst {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = INST_PATTERN
            .captures(s)
            .ok_or_else(|| failure::err_msg("does not match inst pattern"))?;

        let op = match &c[1] {
            "addr" => Op::Addr,
            "addi" => Op::Addi,
            "mulr" => Op::Mulr,
            "muli" => Op::Muli,
            "banr" => Op::Banr,
            "bani" => Op::Bani,
            "borr" => Op::Borr,
            "bori" => Op::Bori,
            "setr" => Op::Setr,
            "seti" => Op::Seti,
            "gtir" => Op::Gtir,
            "gtri" => Op::Gtri,
            "gtrr" => Op::Gtrr,
            "eqir" => Op::Eqir,
            "eqri" => Op::Eqri,
            "eqrr" => Op::Eqrr,
            _ => return Err(failure::err_msg("unknown op")),
        };
        let params = (c[2].parse()?, c[3].parse()?, c[4].parse()?);

        Ok(Inst(op, params))
    }
}

struct Machine {
    ip: usize,
    insts: Vec<Inst>,
    pc: usize,
    registers: [i32; 6],
}

impl Machine {
    fn new(input: &str) -> Machine {
        let mut lines = input.trim().split('\n');
        let ip: usize = lines
            .next()
            .map(|l| IP_PATTERN.captures(l))
            .expect("missing ip declaration")
            .expect("ip declaration does not patch pattern")[1]
            .parse()
            .expect("ip number is not valid");
        let insts = lines
            .map(|l| l.parse::<Inst>().expect("cannot parse inst"))
            .collect::<Vec<_>>();
        let pc = 0usize;
        let registers = [0i32; 6];

        Machine {
            ip,
            insts,
            pc,
            registers,
        }
    }

    #[inline]
    fn step(&mut self) -> bool {
        if self.pc < self.insts.len() {
            self.registers[self.ip] = self.pc as _;
            self.insts[self.pc].exec(&mut self.registers);
            self.pc = self.registers[self.ip] as _;
            self.pc += 1;

            true
        } else {
            false
        }
    }
}

pub fn puzzle1(input: &str) -> i32 {
    let mut m = Machine::new(input);
    while m.step() {}
    m.registers[0]
}

pub fn puzzle2(input: &str) -> i32 {
    let mut m = Machine::new(input);
    m.registers[0] = 1;

    while m.pc != 1 {
        m.step();
    }

    // turns out that when we reach PC 1 that the program just calculates the
    // sum of the factors of what is in register 4, so just do that ourselves as
    // it is faster
    let number = m.registers[4];
    let mut factors = 0;
    for i in 1..=number {
        if number % i == 0 {
            factors += i
        }
    }

    factors
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"
        #ip 0
        seti 5 0 1
        seti 6 0 2
        addi 0 1 0
        addr 1 2 3
        setr 1 0 0
        seti 8 0 4
        seti 9 0 5
    ";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(INPUT), 6);
    }
}
