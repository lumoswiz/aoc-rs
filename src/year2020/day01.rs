use crate::util;
use itertools::Itertools;

pub fn solve(numbers: Vec<i32>, size: u8) -> i32 {
    for combo in numbers.iter().combinations(2) {
        if combo.iter().sum() == 2020 {
            return combo.iter().product();
        }
    }
    0
}

pub fn puzzle1(input: &str) -> i32 {
    let parsed_input = util::split(input)
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    solve(parsed_input, 2)
}

pub fn puzzle2(_input: &str) -> i32 {
    let parsed_input = util::split(input)
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    solve(parsed_input, 3)
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
            514579
        );
    }
}
