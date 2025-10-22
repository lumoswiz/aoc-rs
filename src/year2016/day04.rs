use std::{collections::HashMap, convert::TryInto};

#[derive(Debug)]
pub struct EncryptedData {
    pub words: String,
    pub sector_id: u32,
    pub checksum: [char; 5],
}

#[derive(Debug)]
pub enum ParseErr {
    MissingBracket,
    Overflow,
    BadChecksumLength,
    MissingMixed,
}

pub fn is_lower(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|x| x.is_ascii_lowercase())
}

pub fn parse_mixed(s: &str) -> Result<(u32, [char; 5]), ParseErr> {
    let stripped_last_bracket = s.strip_suffix(']').ok_or(ParseErr::MissingBracket)?;
    let (sector_str, checksum_str) = stripped_last_bracket
        .split_once('[')
        .ok_or(ParseErr::MissingBracket)?;
    let sector_id = sector_str.parse::<u32>().map_err(|_| ParseErr::Overflow)?;
    if checksum_str.len() != 5 {
        return Err(ParseErr::BadChecksumLength);
    }
    let mut it = checksum_str.chars();
    let checksum = [
        it.next().unwrap(),
        it.next().unwrap(),
        it.next().unwrap(),
        it.next().unwrap(),
        it.next().unwrap(),
    ];

    Ok((sector_id, checksum))
}

impl EncryptedData {
    pub fn from_line(line: &str) -> Result<Self, ParseErr> {
        let mut lowers: Vec<&str> = Vec::new();
        let mut mixed: Option<&str> = None;

        for part in line.split('-') {
            if is_lower(part) {
                lowers.push(part);
            } else {
                mixed = Some(part);
            }
        }

        let (sector_id, checksum) = parse_mixed(mixed.ok_or(ParseErr::MissingMixed)?)?;
        let words = lowers.join(" ");

        Ok(Self {
            words,
            sector_id,
            checksum,
        })
    }

    pub fn top_five(&self) -> [char; 5] {
        let mut f: HashMap<char, u32> = HashMap::new();
        for c in self.words.chars() {
            if c == ' ' {
                continue;
            }
            *f.entry(c).or_insert(0) += 1;
        }
        let mut items: Vec<(char, u32)> = f.into_iter().collect();
        items.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

        let top: Vec<char> = items.into_iter().map(|(c, _)| c).take(5).collect();
        top.try_into().unwrap()
    }

    pub fn checksum_matches(&self) -> bool {
        self.top_five() == self.checksum
    }

    fn shift_amount(&self) -> u8 {
        (self.sector_id % 26) as u8
    }

    fn shift_byte(b: u8, k: u8) -> u8 {
        if (b'a'..=b'z').contains(&b) {
            let off = b - b'a';
            b'a' + ((off + k) % 26)
        } else {
            b
        }
    }

    pub fn shifted_words(&self) -> String {
        let k = self.shift_amount();
        let mut out = String::with_capacity(self.words.len());
        for &b in self.words.as_bytes() {
            out.push(Self::shift_byte(b, k) as char)
        }
        out
    }
}

pub fn parse_input(input: &str) -> Result<Vec<EncryptedData>, ParseErr> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(EncryptedData::from_line)
        .collect()
}

pub fn puzzle1(input: &str) -> u32 {
    parse_input(input)
        .unwrap()
        .into_iter()
        .filter(|e| e.checksum_matches())
        .fold(0, |acc, e| acc + e.sector_id)
}

pub fn puzzle2(input: &str) -> u32 {
    parse_input(input)
        .unwrap()
        .into_iter()
        .filter(|e| e.checksum_matches())
        .find_map(|e| {
            let name = e.shifted_words();
            let name = name.to_ascii_lowercase();

            if name.contains("northpole") {
                Some(e.sector_id)
            } else {
                None
            }
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 1514);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 0);
    }
}
