use std::string::ToString;

#[derive(Clone, Copy)]
pub struct DigitString {
    num: i32,
}

pub fn is_sorted(arr: Vec<u32>) -> bool {
    let mut tmp = arr.clone();
    tmp.sort();
    tmp == arr
}

impl DigitString {
    fn to_str(self) -> String {
        self.num.to_string()
    }
    fn digit_list(self) -> Vec<u32> {
        self.to_str().chars().map(|c| c.to_digit(10).unwrap()).collect()
    }
    fn is_ascending(self) -> bool {
        is_sorted(self.digit_list()) // <-- This is the bottle neck!
    }
    fn count_sequence(self) -> Vec<u32> {
        let mut seq = vec![];
        let mut num: u32 = 1;
        for i in 1..self.to_str().len() + 1 {
            if self.to_str().chars().nth(i) == self.to_str().chars().nth(i-1) {
                num += 1;
            } else {
                seq.push(num);
                num = 1;
            }

        }
        seq
    }
}

pub fn puzzle1(input: &str) -> i64 {
    let range = (387638, 919123);

    let mut count = 0;
    for num in range.0..range.1 + 1 {
        let s = DigitString { num };
        if s.is_ascending() && s.count_sequence().into_iter().max().unwrap() > 1 {  // && s.size() == 6
            count += 1;
        }
    }
    count
}

pub fn puzzle2(input: &str) -> i64 {
    let range = (387638, 919123);

    let mut count = 0;
    for num in range.0..range.1 + 1 {
        let s = DigitString { num };

        if s.is_ascending() && s.count_sequence().contains(&2) { // Note && s.size() == 6 is always true
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(""), 0);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }
}
