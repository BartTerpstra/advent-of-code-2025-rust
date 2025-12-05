use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 5)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 5 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

//assert 15 digits max < u128
#[derive(Debug)]
struct Range {lower:u128, upper:u128}

fn as_ranges_and_ingredients(file: &str) -> (Vec<Range>, Vec<u128>){
    let mut ranges = Vec::with_capacity(185);
    let mut ingredients = vec![];
    let trimmed = file.trim();
    let (rangeFile, ingredientFile) = trimmed.split_once('x').unwrap();
    let rangeList = rangeFile.trim().lines();
    let ingredientList = ingredientFile.trim().lines();

    for range in rangeList {
        let (lower, upper) = range.split_once('-').unwrap();
        let (lower, upper) = (lower.parse().unwrap(), upper.parse().unwrap());
        ranges.push(Range{lower, upper})
    }

    for ingredient in ingredientList {
        ingredients.push(ingredient.parse().unwrap());
    }

    (ranges, ingredients)
}

fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    let (ranges, ingredients) = as_ranges_and_ingredients(_input);
    let mut count = 0;
    for ingredient in ingredients {
        for range in &ranges {
            if ingredient <= range.upper && ingredient >= range.lower{
                //in range
                count+=1;
                break;
            }
        }
    }


    Ok(count)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    let (ranges, _) = as_ranges_and_ingredients(_input);
    let mut count = 0;
    for range in &ranges {
        for range in &ranges {
            //for a new range.
            //if range inside existing range
            //expand range
            //else, add to range list.
        }
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
"3-5
10-14
16-20
12-18
x
1
5
8
11
17
32";

    #[test]
    fn test_parser() {
        let (range, ing) = as_ranges_and_ingredients(EXAMPLE);
        println!("{:?}", range );
        println!("{:?}", ing );
        // assert_eq!(result.to_string(), "3");
    }

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "3");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "0");
    }
}
