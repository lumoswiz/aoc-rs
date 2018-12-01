use std::collections::HashSet;

fn parse<'a>(input: &'a str) -> impl 'a + Iterator<Item = i64> {
    input
        .trim()
        .split(&[',', '\n'][..])
        .map(|s| s.trim().trim_start_matches('+'))
        .map(|s| s.parse::<i64>().unwrap())
}

pub fn run1(input: &str) -> i64 {
    parse(input).sum()
}

pub fn run2(input: &str) -> i64 {
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
#[test]
fn test1() {
    assert_eq!(run1("+1, +1, +1"), 3);
    assert_eq!(run1("+1, +1, -2"), 0);
    assert_eq!(run1("-1, -2, -3"), -6);
}

#[cfg(test)]
#[test]
fn test2() {
    assert_eq!(run2("+1, -1"), 0);
    assert_eq!(run2("+3, +3, +4, -2, -4"), 10);
    assert_eq!(run2("-6, +3, +8, +5, -6"), 5);
    assert_eq!(run2("+7, +7, -2, -7, -4"), 14);
}
