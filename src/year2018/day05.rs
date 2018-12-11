use std::cmp;
use std::collections::HashSet;

fn reduce_chain(chain: &mut Vec<u8>) {
    let mut i = 0;
    let mut pos = 1;

    while i != pos {
        i = 0;
        pos = 0;
        while i + 1 < chain.len() {
            if chain[i].is_ascii_uppercase() ^ chain[i + 1].is_ascii_uppercase()
                && chain[i].to_ascii_uppercase() == chain[i + 1].to_ascii_uppercase()
            {
                i += 2;
            } else {
                chain[pos] = chain[i];
                pos += 1;
                i += 1;
            }
        }
        if i < chain.len() {
            chain[pos] = chain[i];
            pos += 1;
            i += 1;
        }

        chain.truncate(pos);
    }
}

pub fn puzzle1(input: &str) -> usize {
    let mut chain = input.trim().as_bytes().to_vec();
    reduce_chain(&mut chain);

    chain.len()
}

pub fn puzzle2(input: &str) -> usize {
    let chain = input.trim().as_bytes();
    let unique_units = chain
        .iter()
        .map(|b| b.to_ascii_uppercase())
        .collect::<HashSet<_>>();

    let mut shortest = chain.len();
    let mut filtered = Vec::<u8>::with_capacity(chain.len());
    for unit in unique_units {
        filtered.clear();
        filtered.extend(chain.iter().filter(|b| b.to_ascii_uppercase() != unit));
        reduce_chain(&mut filtered);
        shortest = cmp::min(filtered.len(), shortest);
    }

    shortest
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
