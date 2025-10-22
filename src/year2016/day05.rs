use md5::{Digest, Md5};

pub struct Password(pub [char; 8]);

fn hex_to_char(n: u8) -> char {
    b"0123456789abcdef"[n as usize] as char
}

fn md5_bytes(base: &str, n: u64) -> [u8; 16] {
    let msg = format!("{base}{n}");
    let digest = Md5::digest(msg.as_bytes());
    digest.into()
}

fn has_five_leading_zero_hex(d: &[u8; 16]) -> bool {
    d[0] == 0 && d[1] == 0 && (d[2] & 0xF0) == 0
}

fn sixth_hex_digit(d: &[u8; 16]) -> u8 {
    d[2] & 0x0F
}

fn seventh_hex_digit(d: &[u8; 16]) -> u8 {
    (d[3] & 0xF0) >> 4
}

impl Password {
    pub fn mine(base: &str) -> Self {
        let mut out = ['\0'; 8];
        let mut i = 0usize;
        let mut n = 0u64;

        while i < 8 {
            let d = md5_bytes(base, n);
            if has_five_leading_zero_hex(&d) {
                out[i] = hex_to_char(sixth_hex_digit(&d));
                i += 1;
            }
            n += 1;
        }

        Password(out)
    }

    pub fn mine_positional(base: &str) -> Self {
        let mut slots: [Option<char>; 8] = [None; 8];
        let mut filled: usize = 0;
        let mut n: u64 = 0;

        while filled < 8 {
            let d = md5_bytes(base, n);
            if has_five_leading_zero_hex(&d) {
                let pos = sixth_hex_digit(&d) as usize;
                if pos < 8 && slots[pos].is_none() {
                    let ch = hex_to_char(seventh_hex_digit(&d));
                    slots[pos] = Some(ch);
                    filled += 1;
                }
            }
            n += 1;
        }

        let mut out = ['\0'; 8];
        for i in 0..8 {
            out[i] = slots[i].unwrap();
        }
        Password(out)
    }

    pub fn to_string(&self) -> String {
        self.0.iter().collect()
    }
}

pub fn puzzle1(input: &str) -> String {
    Password::mine(input.trim()).to_string()
}

pub fn puzzle2(input: &str) -> String {
    Password::mine_positional(input.trim()).to_string()
}

#[cfg(test)]
mod tests {
    const SAMPLE_INPUT: &str = "abc";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), "18f47a30");
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), "05ace8e3");
    }
}
