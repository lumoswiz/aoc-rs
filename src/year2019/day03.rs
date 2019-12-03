use std::str::FromStr;

pub fn parse_input(input: &str) -> Vec<(char, i64)> {
    input.split(',').map(|s| (s.chars().nth(0).unwrap(), i64::from_str(&s[1..]).unwrap())).collect()
}

pub fn puzzle1(input: &str) -> i64 {
    0
}

pub fn puzzle2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        println!("{:?}", super::parse_input("R1000,U371,R195"));
        assert_eq!(1, 0);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }
}
