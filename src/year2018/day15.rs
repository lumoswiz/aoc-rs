struct Area {
    dim: (usize, usize),
    layout: Vec<u8>,
}

impl Area {
    fn new(layout: &str) -> Area {
        let (dim, layout) = layout.trim().split('\n').map(|l| l.trim()).fold(
            ((0, 0), Vec::with_capacity(layout.len())),
            |((_, h), mut layout), line| {
                layout.extend_from_slice(line.as_bytes());
                ((line.len(), h + 1), layout)
            },
        );

        Area { dim, layout }
    }
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
        assert_eq!(super::puzzle1(""), 0);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }
}
