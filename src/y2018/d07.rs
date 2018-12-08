use crate::util;
use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;

struct Step(char, char);

lazy_static! {
    static ref STEP_PATTERN: Regex =
        Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
}

impl FromStr for Step {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = STEP_PATTERN
            .captures(s)
            .ok_or_else(|| failure::err_msg("does not match step pattern"))?;

        Ok(Step(
            c[1].chars().next().unwrap(),
            c[2].chars().next().unwrap(),
        ))
    }
}

struct Node {
    id: char,
    deps: HashSet<char>,
    ns: HashSet<char>,
}

impl Node {
    fn new(id: char) -> Node {
        Node {
            id,
            deps: HashSet::new(),
            ns: HashSet::new(),
        }
    }
}

struct Graph {
    nodes: HashMap<char, Node>,
}

impl Graph {
    fn new<I: Iterator<Item = Step>>(steps: I) -> Graph {
        let mut nodes = HashMap::new();
        for step in steps {
            let Step(dep, n) = step;
            nodes.entry(n).or_insert(Node::new(n)).deps.insert(dep);
            nodes.entry(dep).or_insert(Node::new(dep)).ns.insert(n);
        }

        Graph { nodes }
    }

    fn available<'a>(&'a self) -> impl 'a + Iterator<Item = char> {
        self.nodes.values().filter_map(|n| match n.deps.len() {
            0 => Some(n.id),
            _ => None,
        })
    }

    fn take(&mut self, id: char) -> Option<Node> {
        let node = self.nodes.remove(&id)?;
        if node.deps.len() != 0 {
            self.nodes.insert(id, node);
            return None;
        }

        for nn in node.ns.iter() {
            self.nodes.get_mut(nn).unwrap().deps.remove(&id);
        }

        Some(node)
    }
}

pub fn puzzle1(input: &str) -> String {
    let mut graph = Graph::new(util::parse(input));

    let mut result = String::with_capacity(graph.nodes.len());
    let mut queue = BinaryHeap::new();
    while !graph.nodes.is_empty() {
        queue.extend(graph.available().map(Reverse));
        let next = match queue.pop() {
            Some(Reverse(id)) => {
                queue.clear();
                id
            }
            None => break,
        };

        result.push(next);
        graph.take(next).unwrap();
    }

    result
}

pub fn puzzle2(input: &str) -> usize {
    puzzle2_with_args(input, 5, 60)
}

fn puzzle2_with_args(input: &str, nworkers: usize, overhead: usize) -> usize {
    let mut graph = Graph::new(util::parse(input));

    let mut total_duration = 0;
    let mut workers = Vec::with_capacity(nworkers);
    let mut queued = HashSet::new();
    let mut queue = BinaryHeap::new();
    while !graph.nodes.is_empty() {
        for available in graph.available() {
            if queued.insert(available) {
                queue.push(Reverse(available));
            }
        }
        while workers.len() < nworkers {
            let next = match queue.pop() {
                Some(Reverse(id)) => id,
                None => break,
            };
            workers.push((next, duration(next) + overhead));
        }

        if workers.is_empty() {
            break;
        }

        workers.sort_by_key(|(_, d)| Reverse(*d));
        let (next, complete) = workers.pop().unwrap();
        for (_, ref mut dur) in workers.iter_mut() {
            *dur -= complete;
        }

        total_duration += complete;
        graph.take(next).unwrap();
    }

    total_duration
}

fn duration(c: char) -> usize {
    ((c as u8) - b'A' + 1) as _
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"
        Step C must be finished before step A can begin.
        Step C must be finished before step F can begin.
        Step A must be finished before step B can begin.
        Step A must be finished before step D can begin.
        Step B must be finished before step E can begin.
        Step D must be finished before step E can begin.
        Step F must be finished before step E can begin.
    ";

    #[test]
    fn puzzle1() {
        assert_eq!(&super::puzzle1(INPUT), "CABDFE");
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2_with_args(INPUT, 2, 0), 15);
    }
}
