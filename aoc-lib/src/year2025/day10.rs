use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 10)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 10 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

struct BinaryValue{
    values:Vec<bool>,
    size:u8
}
struct Machine{
    lights:Light,
    Buttons:Vec<Button>,
    size:u8
}

struct Light{
    value: String,
    binary: BinaryValue
}

struct Button{
    value: String,
    binary: BinaryValue
}
// fn as_light(part: &str) -> Light{
//
// }
// fn as_button(part: &str) -> Button{
//
// }

fn as_machines(file:&str)->Vec<Machine>{
    let result = vec![];
    let lines = file.trim().lines();
    for line in lines {
        let mut words = line.split_whitespace();
        let lights = words.next().unwrap();


    }
    result
}


fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {

    let val = BinaryValue{values:vec![], size: 5 };
    //read and xor values until you have found a working combination.
    //very nice runtime properties of n+n-1+n-2, etc.

    Ok(0)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 2
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "0");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "0");
    }
}
