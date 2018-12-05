fn parse<'a>(input: &'a str) -> impl 'a + Iterator<Item = &'a str> {
    input.trim().split(&[',', '\n'][..]).map(|s| s.trim())
}

pub fn problem1(input: &str) -> i64 {
    0
}

pub fn problem2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem1() {
        assert_eq!(super::problem1(""), 0);
    }

    #[test]
    fn problem2() {
        assert_eq!(super::problem2(""), 0);
    }
}
