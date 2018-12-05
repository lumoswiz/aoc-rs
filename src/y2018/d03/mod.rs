use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

lazy_static! {
    static ref CLAIM_PATTERN: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
}

fn parse<'a>(input: &'a str) -> impl 'a + Iterator<Item = (i64, Rect)> {
    input
        .trim()
        .split('\n')
        .map(|s| CLAIM_PATTERN.captures(s.trim()).unwrap())
        .map(|c| {
            (
                c[1].parse().unwrap(),
                Rect {
                    x: c[2].parse().unwrap(),
                    y: c[3].parse().unwrap(),
                    w: c[4].parse().unwrap(),
                    h: c[5].parse().unwrap(),
                },
            )
        })
}

pub fn problem1(input: &str) -> usize {
    parse(input)
        .map(|(_, r)| {
            (r.x..(r.x + r.w))
                .map(move |i| (r.y..(r.y + r.h)).map(move |j| (i, j)))
                .flatten()
        })
        .flatten()
        .fold(
            (HashMap::new(), 0usize),
            |(mut claimed, mut overlap), pos| {
                {
                    let sqin = claimed.entry(pos).or_insert(0u8);
                    *sqin += 1;
                    if *sqin == 2 {
                        overlap += 1;
                    }
                }
                (claimed, overlap)
            },
        )
        .1
}

pub fn problem2(input: &str) -> i64 {
    let claims = parse(input).collect::<Vec<_>>();
    let ids = claims.iter().map(|(id, _)| *id).collect::<HashSet<_>>();

    let len = claims.len();
    let mut no_overlap = (0..len)
        .map(|i| ((i + 1)..len).map(move |j| (i, j)))
        .flatten()
        .fold(ids, |mut ids, (i, j)| {
            let (ref id1, ref r1) = &claims[i];
            let (ref id2, ref r2) = &claims[j];

            if r1.x + r1.w > r2.x && r1.x < r2.x + r2.w && r1.y + r1.h > r2.y && r1.y < r2.y + r2.h
            {
                ids.remove(id1);
                ids.remove(id2);
            }

            ids
        });

    assert_eq!(no_overlap.len(), 1);
    let result = { no_overlap.drain().nth(0).unwrap() };

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem1() {
        assert_eq!(
            super::problem1(
                r"
                #1 @ 1,3: 4x4
                #2 @ 3,1: 4x4
                #3 @ 5,5: 2x2
                "
            ),
            4
        );
    }

    #[test]
    fn problem2() {
        assert_eq!(
            super::problem2(
                r"
                #1 @ 1,3: 4x4
                #2 @ 3,1: 4x4
                #3 @ 5,5: 2x2
                "
            ),
            3
        );
    }
}
