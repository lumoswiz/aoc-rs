pub fn puzzle1(input: &str) -> i64 {
    2
}

pub fn puzzle2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1("1"), 1);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }
}
