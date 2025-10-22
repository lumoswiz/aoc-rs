use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day3Errors {
    #[error("expected 3 numbers")]
    NotThreeNumbers,
    #[error("expected multiple of 3")]
    NotMultipleThree,
    #[error("parse int error")]
    Day3ParseIntError(#[from] ParseIntError),
}

fn parse_input(input: &str) -> Result<Vec<[u32; 3]>, Day3Errors> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let nums = line
                .split_whitespace()
                .map(|s| s.parse::<u32>())
                .collect::<Result<Vec<u32>, _>>()?;
            match nums.as_slice() {
                [a, b, c] => Ok([*a, *b, *c]),
                _ => Err(Day3Errors::NotThreeNumbers),
            }
        })
        .collect()
}

pub fn transform(rows: &[[u32; 3]]) -> Result<Vec<[u32; 3]>, Day3Errors> {
    let mut out = Vec::with_capacity(rows.len());
    let mut chunks = rows.chunks_exact(3);
    for chunk in &mut chunks {
        let (r1, r2, r3) = (chunk[0], chunk[1], chunk[2]);
        for i in 0..3 {
            let mut t = [r1[i], r2[i], r3[i]];
            t.sort();
            out.push(t);
        }
    }
    if !chunks.remainder().is_empty() {
        return Err(Day3Errors::NotMultipleThree);
    }

    Ok(out)
}

pub fn puzzle1(input: &str) -> u32 {
    let rows = parse_input(input).unwrap();
    rows.into_iter()
        .filter(|t| {
            let mut x = *t;
            x.sort();
            x[0] + x[1] > x[2]
        })
        .count() as u32
}

pub fn puzzle2(input: &str) -> u32 {
    let rows = parse_input(input).unwrap();
    let cols = transform(&rows).unwrap();
    cols.into_iter().filter(|t| t[0] + t[1] > t[2]).count() as u32
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "5 10 25";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 0);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 0);
    }
}
