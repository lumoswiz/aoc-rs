use std::collections::BTreeSet;
use std::iter;

pub fn puzzle1(input: &str) -> usize {
    let chain = input.trim().as_bytes();
    let mut reductions = BTreeSet::new();
    let mut new_reductions = Vec::new();

    loop {
        {
            let mut start = 0;
            let mut indices = reductions
                .iter()
                .cloned()
                .chain(iter::once(chain.len()))
                .map(|i| {
                    if i > start {
                        let s = start;
                        start = i + 2;
                        (s..i)
                    } else {
                        start += 2;
                        (0..0)
                    }
                })
                .flatten();

            let mut i1 = match indices.next() {
                Some(i) => i,
                None => return 0,
            };
            while let Some(i2) = indices.next() {
                i1 = if chain[i1].is_ascii_uppercase() ^ chain[i2].is_ascii_uppercase()
                    && chain[i1].to_ascii_uppercase() == chain[i2].to_ascii_uppercase()
                {
                    new_reductions.push(i1);
                    match indices.next() {
                        Some(i) => i,
                        None => break,
                    }
                } else {
                    i2
                };
            }
        }

        if new_reductions.is_empty() {
            break;
        }

        for reduction in new_reductions.drain(..) {
            reductions.insert(reduction);
        }
    }

    chain.len() - reductions.len() * 2
}

pub fn puzzle2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }
}
