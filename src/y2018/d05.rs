use std::cmp;
use std::collections::{BTreeMap, HashSet};
use std::iter;

fn reduce_chain<I: Iterator<Item = usize>>(input: &str, skip: I) -> usize {
    let chain = input.trim().as_bytes();
    let mut reductions = skip.map(|s| (s, 1usize)).collect::<BTreeMap<_, _>>();
    let mut pairs = Vec::new();

    loop {
        {
            let mut start = 0;
            let mut indices = reductions
                .iter()
                .map(|(i, l)| (*i, *l))
                .chain(iter::once((chain.len(), 0)))
                .map(|(i, l)| {
                    if i > start {
                        let s = start;
                        start = i + l;
                        (s..i)
                    } else {
                        start += l;
                        (0..0)
                    }
                })
                .flatten();

            let mut i1 = match indices.next() {
                Some(i) => i,
                None => return 0,
            };
            while let Some(i2) = indices.next() {
                i1 = if chain[i1].is_ascii_lowercase() ^ chain[i2].is_ascii_lowercase()
                    && chain[i1].to_ascii_lowercase() == chain[i2].to_ascii_lowercase()
                {
                    pairs.push(i1);
                    match indices.next() {
                        Some(i) => i,
                        None => break,
                    }
                } else {
                    i2
                };
            }
        }

        if pairs.is_empty() {
            break;
        }

        for i in pairs.drain(..) {
            reductions.insert(i, 2);
        }
    }

    let total_reductions: usize = reductions.values().cloned().sum();
    chain.len() - total_reductions
}

pub fn puzzle1(input: &str) -> usize {
    reduce_chain(input, iter::empty())
}

pub fn puzzle2(input: &str) -> usize {
    let chain = input.trim().as_bytes();
    let unique_units = chain
        .iter()
        .map(|b| b.to_ascii_lowercase())
        .collect::<HashSet<_>>();

    let mut min = None;
    for unit in unique_units {
        let skip = chain
            .iter()
            .enumerate()
            .filter(|(_, c)| c.to_ascii_lowercase() == unit)
            .map(|(i, _)| i);
        let reduced_len = reduce_chain(input, skip);

        min = min.map(|m| cmp::min(m, reduced_len)).or(Some(reduced_len));
    }

    min.unwrap_or(chain.len())
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2("dabAcCaCBAcCcaDA"), 4);
    }
}
