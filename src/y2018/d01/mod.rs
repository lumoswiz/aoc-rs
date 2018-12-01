use std::collections::HashSet;

fn parse<'a>(input: &'a str) -> impl 'a + Iterator<Item = i64> {
    input
        .trim()
        .split(&[',', '\n'][..])
        .map(|s| s.trim().trim_start_matches('+'))
        .map(|s| s.parse::<i64>().unwrap())
}

pub fn problem1(input: &str) -> i64 {
    parse(input).sum()
}

pub fn problem2(input: &str) -> i64 {
    let mut sum = 0;
    let mut numbers = HashSet::new();

    numbers.insert(0);
    loop {
        for n in parse(input) {
            sum += n;
            if !numbers.insert(sum) {
                return sum;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem1() {
        assert_eq!(super::problem1("+1, +1, +1"), 3);
        assert_eq!(super::problem1("+1, +1, -2"), 0);
        assert_eq!(super::problem1("-1, -2, -3"), -6);
    }

    #[test]
    fn problem2() {
        assert_eq!(super::problem2("+1, -1"), 0);
        assert_eq!(super::problem2("+3, +3, +4, -2, -4"), 10);
        assert_eq!(super::problem2("-6, +3, +8, +5, -6"), 5);
        assert_eq!(super::problem2("+7, +7, -2, -7, -4"), 14);
    }
}
