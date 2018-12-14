use std::str;

pub fn puzzle<F>(size_hint: usize, cond: F) -> String
where
    F: Fn(&str) -> bool,
{
    let mut scoreboard = String::with_capacity(size_hint);
    scoreboard.push('3');
    scoreboard.push('7');
    let mut elfs = (0, 1);

    fn advance(pos: usize, score: u8, len: usize) -> usize {
        (pos + 1 + (score as usize)) % len
    }

    while !cond(&scoreboard) {
        let scoreboard = unsafe { scoreboard.as_mut_vec() };

        let scores = (scoreboard[elfs.0] - b'0', scoreboard[elfs.1] - b'0');

        let sum = scores.0 + scores.1;
        if sum < 10 {
            scoreboard.push(sum + b'0');
        } else {
            scoreboard.push(b'1');
            scoreboard.push(sum - 10 + b'0');
        }

        let len = scoreboard.len();
        elfs = (
            advance(elfs.0, scores.0, len),
            advance(elfs.1, scores.1, len),
        );
    }

    scoreboard
}

pub fn puzzle1(input: &str) -> String {
    let recipes: usize = input.trim().parse().expect("failed to parse input");
    let total_recipes = recipes + 10;

    let scoreboard = puzzle(total_recipes + 1, |s| s.len() >= total_recipes);
    scoreboard[recipes..total_recipes].to_owned()
}

pub fn puzzle2(input: &str) -> usize {
    let input = input.trim();

    fn slice_end(s: &str, len: usize) -> &str {
        let start = if s.len() <= len { 0 } else { s.len() - len - 1 };
        &s[start..]
    }

    let scoreboard = puzzle(0x400000, |s| slice_end(&s, input.len()).contains(input));
    scoreboard.rfind(input).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1("9"), "5158916779");
        assert_eq!(super::puzzle1("5"), "0124515891");
        assert_eq!(super::puzzle1("18"), "9251071085");
        assert_eq!(super::puzzle1("2018"), "5941429882");
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2("51589"), 9);
        assert_eq!(super::puzzle2("01245"), 5);
        assert_eq!(super::puzzle2("92510"), 18);
        assert_eq!(super::puzzle2("59414"), 2018);
    }
}
