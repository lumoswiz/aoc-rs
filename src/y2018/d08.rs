#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
}

impl Node {
    fn parse(input: &str) -> Node {
        let mut values = input
            .trim()
            .split(' ')
            .map(|s| s.trim().parse::<i32>().unwrap());
        Node::read(&mut values)
    }

    fn read<I: Iterator<Item = i32>>(values: &mut I) -> Node {
        let nchildren = values.next().expect("missing children count");
        let nmetadata = values.next().expect("missing metadata count");

        Node {
            children: (0..nchildren).map(|_| Node::read(values)).collect(),
            metadata: (0..nmetadata)
                .map(|_| values.next().expect("missing metadata value"))
                .collect(),
        }
    }

    fn sum(&self) -> i32 {
        self.metadata
            .iter()
            .cloned()
            .chain(self.children.iter().map(|n| n.sum()))
            .sum()
    }

    fn value(&self) -> i32 {
        let nchildren = self.children.len() as i32;
        match nchildren {
            0 => self.sum(),
            _ => self
                .metadata
                .iter()
                .filter(|&&i| i > 0 && i <= nchildren)
                .map(|i| self.children[(i - 1) as usize].value())
                .sum(),
        }
    }
}

pub fn puzzle1(input: &str) -> i32 {
    let root = Node::parse(input);
    root.sum()
}

pub fn puzzle2(input: &str) -> i32 {
    let root = Node::parse(input);
    root.value()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(INPUT), 138);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(INPUT), 66);
    }
}
