use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2024, 2)?;

    let reports = to_reports(&input)?;
    let part1 = solve_part1(&reports)?;
    let part2 = solve_part2(&reports)?;

    println!("Day 2 / Year 2024");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn solve_part1(_input: &Vec<Vec<i8>>) -> Result<impl std::fmt::Display> {
    let total = _input
        .iter()
        .map(|x|strict_save_report(x))
        .map(|x|{println!("{:#?}", x);x})
        .map(|x|x.unwrap())
        .filter(|&x| x).count();
    Ok(total.to_string())
}

fn solve_part2(_input: &Vec<Vec<i8>>) -> Result<impl std::fmt::Display> {
    let total = _input
        .iter()
        .map(|x|strict_save_report(x))
        .map(|x|{println!("{:#?}", x);x})
        .map(|x|x.unwrap())
        .filter(|&x| x).count();
    Ok(total.to_string())
}

fn strict_save_report(report: &Vec<i8>) -> Result<bool>{
    anyhow::ensure!(report.len() > 2, "Report must have atleast 2 values!");

    if report[0] < report[1] {
        //ascending
        for n in 0..report.len()-1{
            let diff =report[n] - report[n+1];
            //should be between -1 and -3 or return false
            if diff< -3 || -1<diff {
                return Ok(false)
            }
        }
        return Ok(true);
    } else if report[0] > report[1]{
        //descending
        for n in 0..report.len()-1 {
            let diff = report[n] - report[n + 1];
            //should be between 1 and 3 or return false
            if diff < 1 || 3 < diff {
                return Ok(false)
            }
        }
       return Ok(true);
    }
    //first 2 numbers are equal
    Ok(false)
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
        let reports = to_reports(EXAMPLE).unwrap();
        let result = solve_part1(&reports).unwrap();
        assert_eq!(result.to_string(), "2");
    }

    #[test]
    fn test_part2() {
        let reports = to_reports(EXAMPLE).unwrap();
        let result = solve_part1(&reports).unwrap();
        assert_eq!(result.to_string(), "4");
    }
}
