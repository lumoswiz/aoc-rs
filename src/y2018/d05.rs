use std::collections::HashSet;

pub fn puzzle1(input: &str) -> usize {
    let mut chain = input.trim().to_string();

    loop {
        let mut reduction = None;
        for ((i, c1), c2) in chain.chars().enumerate().zip(chain[1..].chars()) {
            if c1.is_ascii_uppercase() ^ c2.is_ascii_uppercase()
                && c1.to_ascii_uppercase() == c2.to_ascii_uppercase()
            {
                reduction = Some(i);
                break;
            }
        }

        match reduction {
            Some(i) => {
                chain.drain(i..=i + 1);
            }
            None => break,
        }
    }

    chain.len()
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
