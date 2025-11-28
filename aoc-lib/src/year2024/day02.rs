use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2024, 2)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 2 / Year 2024");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 1
    Ok(0)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 2
    Ok(0)
}

fn to_reports(content: &str) -> Result<Vec<Vec<i8>>> {
    let mut reports: Vec<Vec<i8>>= vec![];
    for line in content.trim().lines() {
        let mut report = vec![];
        let mut parts = line.split_whitespace();
        while let Some(value) = parts.next() {
            report.push(value.parse::<i8>()?);
        }
        reports.push(report)
    }
    Ok(reports)
}
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";

    #[test]
    fn test_parsing() {
        let result = to_reports(EXAMPLE).unwrap();
        assert_eq!(result[0][0], 7);
        assert_eq!(result[1][1], 2);
        assert_eq!(result[2][2], 6);
        assert_eq!(result[3][3], 4);
        assert_eq!(result[4][4], 1);
    }

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "269");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "337");
    }
}
