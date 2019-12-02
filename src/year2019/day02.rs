use std::str::FromStr;

pub fn puzzle1(input: &str) -> i64 {
    let mut numbers: Vec<i64> = input.split(",").map(|s| i64::from_str(s.trim()).unwrap()).collect();
    println!("{:?}", numbers);
    let mut i: usize = 0;
    while numbers[i] != 99 {
        let op = numbers[i];
        let x = numbers[i + 1] as usize;
        let y = numbers[i + 2] as usize;
        let z = numbers[i + 3] as usize;
        println!("Updating position {} with operation {} between {} and {}", z, op, numbers[x], numbers[y]);
        numbers[z] = match op {
            1 => numbers[x] + numbers[y],
            2 => numbers[x] * numbers[y],
            _ => panic!("Something went wrong with op parsing!")
        };
        println!("Finished with index {}", i);
        i += 4;

    }

    numbers[0]
}

pub fn puzzle2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
//        assert_eq!(super::puzzle1("1,9,10,3,2,3,11,0,99,30,40,50"), 3500);
//        assert_eq!(super::puzzle1("1,0,0,0,99"), 2);
//        assert_eq!(super::puzzle1("2,3,0,3,99"), 2);
//        assert_eq!(super::puzzle1("2,4,4,5,99,0"), 2);
        assert_eq!(super::puzzle1("1,1,1,4,99,5,6,0,99"), 31);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }
}
