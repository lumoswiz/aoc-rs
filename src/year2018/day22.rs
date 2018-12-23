use crate::util::{self, Grid};
use failure::Error;
use lazy_static::lazy_static;
use nalgebra::Point2;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::str::FromStr;
use std::usize;

#[derive(Debug)]
struct CaveProperies {
    depth: usize,
    target: (usize, usize),
}

lazy_static! {
    static ref CAVE_PATTERN: Regex = Regex::new(r"depth: (\d+)\ntarget: (\d+),(\d+)").unwrap();
}

impl FromStr for CaveProperies {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = CAVE_PATTERN
            .captures(s)
            .ok_or_else(|| failure::err_msg("does not match cave pattern"))?;

        Ok(CaveProperies {
            depth: c[1].parse()?,
            target: (c[2].parse()?, c[3].parse()?),
        })
    }
}

struct Cave {
    total_risk: usize,
    grid: Grid,
    target: Point2<usize>,
}

impl Cave {
    fn new(properties: CaveProperies, extra: usize) -> Cave {
        let mut total_risk = 0;
        let (w, h) = (
            properties.target.0 + extra + 1,
            properties.target.1 + extra + 11,
        );
        let mut grid = Grid::new(w, h);
        let target = Point2::new(properties.target.0, properties.target.1);

        let mut erosion_levels = vec![0usize; w * h];
        for y in 0..h {
            for x in 0..w {
                let i = x + y * w;

                let is_target = (x, y) == properties.target;
                let geological_index = match (x, y) {
                    (0, 0) => 0,
                    (_, _) if is_target => 0,
                    (x, 0) => x * 16807,
                    (0, y) => y * 48271,
                    (_, _) => erosion_levels[i - 1] * erosion_levels[i - w],
                };
                let erosion_level = (geological_index + properties.depth) % 20183;
                let region_kind = erosion_level % 3;

                erosion_levels[i] = erosion_level;
                if x <= properties.target.0 && y <= properties.target.1 {
                    total_risk += region_kind;
                }

                grid[[x, y]] = match region_kind {
                    _ if is_target => b'T',
                    0 => b'.',
                    1 => b'=',
                    2 => b'|',
                    _ => unreachable!(),
                }
            }
        }

        Cave {
            total_risk,
            grid,
            target,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

impl Tool {
    fn r(region: u8) -> u8 {
        match region {
            b'.' | b'T' => b'.',
            b'=' => b'=',
            b'|' => b'|',
            _ => unreachable!(),
        }
    }

    fn usable_tool(region1: u8, region2: u8) -> Option<Tool> {
        match (Tool::r(region1), Tool::r(region2)) {
            (b'.', b'|') | (b'|', b'.') => Some(Tool::Torch),
            (b'.', b'=') | (b'=', b'.') => Some(Tool::ClimbingGear),
            (b'=', b'|') | (b'|', b'=') => Some(Tool::Neither),
            _ => None,
        }
    }

    fn can_use(self, region: u8) -> bool {
        match (self, Tool::r(region)) {
            (Tool::Torch, b'.') => true,
            (Tool::Torch, b'|') => true,
            (Tool::ClimbingGear, b'.') => true,
            (Tool::ClimbingGear, b'=') => true,
            (Tool::Neither, b'=') => true,
            (Tool::Neither, b'|') => true,
            _ => false,
        }
    }
}

pub fn puzzle1(input: &str) -> usize {
    let properties = input
        .parse::<CaveProperies>()
        .expect("failed to parse input");
    let cave = Cave::new(properties, 0);

    cave.total_risk
}

pub fn puzzle2(input: &str) -> usize {
    let properties = input
        .parse::<CaveProperies>()
        .expect("failed to parse input");
    let cave = Cave::new(properties, 10);

    println!("{:?}", cave.grid);

    let mut pending = Vec::new();
    let mut visited = HashMap::new();
    let mut min_duration = usize::MAX;

    pending.push((Point2::new(0, 0), Tool::Torch, 0));
    while let Some((pos, tool, duration)) = pending.pop() {
        if pos == cave.target {
            min_duration = cmp::min(
                min_duration,
                match tool {
                    Tool::Torch => duration,
                    _ => duration + 7,
                },
            );
            continue;
        }

        let last_duration = visited.entry((pos, tool)).or_insert(usize::MAX);
        if *last_duration <= duration {
            continue;
        }
        *last_duration = duration;

        let current = cave.grid[pos];
        for p in util::adjacent4(pos) {
            let next = match cave.grid.get(p) {
                Some(n) => n,
                None => continue,
            };

            if tool.can_use(next) {
                pending.push((p, tool, duration + 1));
            }
            if let Some(new_tool) = Tool::usable_tool(current, next) {
                pending.push((p, new_tool, duration + 7 + 1));
            }
        }

        pending.sort_unstable_by(|a, b| b.2.cmp(&a.2));
    }

    min_duration
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1("depth: 510\ntarget: 10,10"), 114);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2("depth: 510\ntarget: 10,10"), 45);
    }
}
