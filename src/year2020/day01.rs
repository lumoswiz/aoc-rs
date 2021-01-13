use crate::util;

pub fn solve(mut numbers: Vec<i64>) -> i64 {
    println!("{:?}", numbers);
    0
}

pub fn puzzle1(input: &str) -> i64 {
    let input = util::split(input);
    solve(input)
}

pub fn puzzle2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {

    #[test]
    fn puzzle1() {
        assert_eq!(
            super::puzzle1(
                r"1721
                        979
                        366
                        299
                        675
                        1456
                "
            ),
            0
        );
    }
}
