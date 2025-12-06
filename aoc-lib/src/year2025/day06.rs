use std::fmt::Formatter;
use crate::utils;
use anyhow::Result;
use colored::Colorize;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 6)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 6 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[derive(Debug)]
struct Formula{
    factors: Vec<Factor>,
    operation: char //'*'|'+'
}

type Factor = u32;

struct CephalopodFormula {
    digits: Vec<Vec<char>>,
    operation:char,
    index:usize,
}

fn as_associate_columns(file: &str)->Vec<Formula>{
    //assert no invalid input
    let mut result = vec![];
    let mut operators = vec![];

    let mut lines = file.trim().lines().rev();
    let operator_line = lines.next().unwrap().split_whitespace();
    for word in operator_line {
        //every word ought to be a string containing a single char operator
        operators.push(word.chars().next().unwrap())
    }

    let width = operators.len();
    //pre-allocate
    for index in 0..width{
        let factors = Vec::with_capacity(6);
        result.push(Formula{factors, operation:operators[index]})
    }

    for line in lines {
        let words = line.split_whitespace();
        let mut index = 0;
        for word in words {
            let value = word.parse().unwrap();
            let formula = result.get_mut(index).unwrap(); //safe because of assert valid input
            let inner = &mut formula.factors;
            inner.push(value);

            index+=1;
        }
    }
    return result
}

fn as_triple_slices(file: &str) ->Vec<Formula>{
    let result = vec![];
    //trim.lines
    //reverse
    //for chars
    //if operator
    //save index with empty formula (cephalopod formula)

    //for rest of lines
    //for every formula
    //index into line and populate
    //spaces are zeroes

    //rotate cephalopod formulas into regular formulas //seperate func
    result
}

fn reduce(formulas: Vec<Formula>) -> u128{
    let mut total = 0;
    for formula in formulas {
        let mut sum: u128 = 0;
        if formula.operation == '*'{
            sum = 1;
        }

        for factor in formula.factors {
            if formula.operation == '*'{
                sum*=factor as u128
            }else{
                sum+=factor as u128
            }
        }
        total+=sum;
        println!("{}", sum)
    }
    total
}

fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    let formulas = as_associate_columns(_input);
    let answer = reduce(formulas);
    Ok(answer)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    let formulas = as_triple_slices(_input);
    let answer = reduce(formulas);
    Ok(answer)
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
    fn parser() {
        let result = as_associate_columns(EXAMPLE);
        println!("{:?}", result);
    }

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
