use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 2)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 2 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[derive(Debug)]
struct Range{
    lower: u64,
    upper: u64
}
fn as_range_list(file: &str)->Vec<Range> {
    //assert no invalid data
    let mut ranges = vec![];
    for range in file.trim().split(',') {
        let mut parts = range.split('-');
        println!("{}", range);

        let lower = parts.next().unwrap().parse::<u64>().unwrap();
        let upper = parts.next().unwrap().parse::<u64>().unwrap();
        ranges.push(Range{lower, upper});
    }
    return ranges
}

fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 1
    //assert smaller than 11 digits, < u64
    let ranges = as_range_list(_input);

    //split every range that crosses 10^n such that every range has 1 uniform digit length
    //sort
    // collapse? are we expecting overlapping ranges? (+count collapses for fun).

    //option a: brute force

    //option b: generators and cutters
    //divisors of digit count are the target repetition lengths
    //i.e. 10 has 2,5 and 1, not 3.
    //the n significant digits repeated, where n is a repetition length, can be the bounds
    //for every range length 0..=10
        //get prime divors
        //for every range
            //for every prime divisor
                //generate list given bounds and primacy
                //sum+=list.sum()

    //optim a: generate compiletime table of all combinations
    Ok(0)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 2
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";


    #[test]
    fn parser() {
        let result = as_range_list(EXAMPLE);
        println!("{:?}",result)
    }

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "1227775554");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "0");
    }
}
