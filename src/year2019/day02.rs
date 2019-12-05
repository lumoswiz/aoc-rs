use std::str::FromStr;
use crate::util::parse_csv;

pub fn solve(mut numbers: Vec<i64>) -> i64 {
    let mut i: usize = 0;
    while numbers[i] != 99 {
        let op = numbers[i];
        let x = numbers[i + 1] as usize;
        let y = numbers[i + 2] as usize;
        let z = numbers[i + 3] as usize;
        numbers[z] = match op {
            1 => numbers[x] + numbers[y],
            2 => numbers[x] * numbers[y],
            _ => panic!("Something went wrong with op parsing!")
        };
        i += 4;
    }
    numbers[0]
}

pub fn puzzle1(input: &str) -> i64 {
    let mut numbers: Vec<i64> = parse_csv(input);
    // Random extra condition in last paragraph
    numbers[1] = 12;
    numbers[2] = 2;
    solve(numbers)
}

pub fn puzzle2(input: &str) -> i64 {
    let mut numbers: Vec<i64> = parse_csv(input);
    let mut res = -1;
    for noun in 0..99 {
        for verb in 0..99 {
            numbers[1] = i64::from(noun);
            numbers[2] = i64::from(verb);
            if solve(numbers.clone()) == 19_690_720 {
                let string_result = format!("{}{}", noun, verb);
                res = i64::from_str(&string_result).expect("Error parsing");
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    pub fn test_solve(input: &str) -> i64 {
        let numbers: Vec<i64> = parse_csv(input);
        solve(numbers)
    }

    #[test]
    fn puzzle1() {
        assert_eq!(super::test_solve("1,9,10,3,2,3,11,0,99,30,40,50"), 3500);
        assert_eq!(super::test_solve("1,0,0,0,99"), 2);
        assert_eq!(super::test_solve("2,3,0,3,99"), 2);
        assert_eq!(super::test_solve("2,4,4,5,99,0"), 2);
        assert_eq!(super::test_solve("1,1,1,4,99,5,6,0,99"), 30);
    }
}
