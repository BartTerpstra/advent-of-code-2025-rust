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

fn to_reports(content: &str) -> Result<Vec<Vec<u8>>> {
    let mut reports: Vec<Vec<u8>>= vec![];
    for line in content.trim().lines() {
        let mut report = vec![];
        let mut parts = line.split_whitespace();
        if let Some(value) = parts.next() {
            report.push(value.parse::<u8>()?);
        }
        reports.push(report)
    }
    Ok(reports)
}
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

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
