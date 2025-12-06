use std::fmt::Formatter;
use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 6)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 6 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

type Formula= Vec<Factor>;
struct Factor {
    value: u32,
    operation: char //'*'|'+'
}

fn as_associate_columns(file: &str)->Vec<Formula>{
    let result = vec![];
    //trim.as lines
    //reverse
    //operation vec
    //for every line
    //zipper
    //return
    return result
}

fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    let formulas = as_associate_columns(_input);
    //for every formula keep an accumulator,
    //for ever factor, operate on accumulator

    // TODO: Implement part 1
    Ok(0)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 2
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = 
 "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "4277556");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "0");
    }
}
