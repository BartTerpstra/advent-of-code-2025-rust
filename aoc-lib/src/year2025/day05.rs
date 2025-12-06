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
struct Range {
    lower: u128,
    upper: u128,
}

fn as_ranges_and_ingredients(file: &str) -> (Vec<Range>, Vec<u128>) {
    let mut ranges = Vec::with_capacity(185);
    let mut ingredients = vec![];
    let trimmed = file.trim();
    //todo fix this deviation where i replace empty new-line split with x, also in source
    let (range_file, ingredient_file) = trimmed.split_once('x').unwrap();
    let range_list = range_file.trim().lines();
    let ingredient_list = ingredient_file.trim().lines();

    for range in range_list {
        let (lower, upper) = range.split_once('-').unwrap();
        let (lower, upper) = (lower.parse().unwrap(), upper.parse().unwrap());
        ranges.push(Range { lower, upper })
    }

    for ingredient in ingredient_list {
        ingredients.push(ingredient.parse().unwrap());
    }

    (ranges, ingredients)
}

fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    let (ranges, ingredients) = as_ranges_and_ingredients(_input);
    let mut count = 0;
    for ingredient in ingredients {
        for range in &ranges {
            if ingredient <= range.upper && ingredient >= range.lower {
                //in range
                count += 1;
                break;
            }
        }
    }

    Ok(count)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    let (mut ranges, _) = as_ranges_and_ingredients(_input);
    loop {
        let mut changed = false;
        for i_candidate in 0..ranges.len() {
            if changed {break;}
            for i_target in 0..ranges.len() {
                if changed {break;}
                if i_candidate == i_target {continue}
                //if range touches other range (always smaller range asks bigger range to be included)
                let targeted = ranges.get(i_target).unwrap();
                let candidate = ranges.get(i_candidate).unwrap();
                let upper_with_kiss = targeted.upper + 1;
                let lower_with_kiss = targeted.lower - 1;
                let lower = candidate.lower;
                let upper = candidate.upper;

                //if touching or inside
                if lower >= lower_with_kiss && lower <= upper_with_kiss ||
                    upper >= lower_with_kiss && upper <= upper_with_kiss{
                    //find smallest and largest.
                    let new_lower = if targeted.lower < candidate.lower {targeted.lower}else{candidate.lower};
                    let new_upper = if targeted.upper > candidate.upper {targeted.upper}else{candidate.upper};
                    //removing by indexes depends on vec not changing ahead of it!
                    let (first,second) = if i_target > i_candidate {(i_target, i_candidate)}else{(i_candidate,i_target)};
                    ranges.remove(first);
                    ranges.remove(second);
                    ranges.push(Range{lower:new_lower, upper:new_upper});
                    changed = true;
                    //break out of candidate (can be done fancy)
                }
            }
        }
        if !changed {break};
    }
    let mut sum: u128 = 0;
    for range in ranges {
        sum += range.upper - range.lower + 1;
    }
    Ok(sum)
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
    }

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "3");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "14");
    }
}
