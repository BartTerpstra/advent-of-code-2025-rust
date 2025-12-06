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

fn transform_to_human(forms: Vec<CephalopodFormula>) -> Vec<Formula>{
    let result = vec![];

    result
}
fn read_as_cephalopod(file: &str) ->Vec<Formula>{
    //assert no invalid input
    let mut cephalopod_forms = vec![];

    let mut lines = file.trim().lines().rev();
    let operator_line = lines.next().unwrap();
    for operation in ['*','+']{
        let indices:Vec<usize> = operator_line.match_indices(operation).map(|x|x.0).collect();
        for index in indices {
            let form = CephalopodFormula{index, operation, digits:vec![]};
            cephalopod_forms.push(form);
        }
    }

    //index into line and populate
    //spaces are zeroes
    for line in lines {
        let char_line: Vec<char> = line.chars().collect();
        for form in &mut cephalopod_forms {
            let mut digit_line = vec![];
            //couldn't quickly think of a prettier way of doing this
            let mut index = form.index;
            //wait to hit characters
            loop{
                let char = *char_line.get(index).unwrap();
                if char == ' ' {
                    digit_line.push('0');
                    index+=1
                }else{
                    digit_line.push(char);
                    index+=1;
                    break;
                }
            }
            //wait to hit end
            loop{
                let char = *char_line.get(index).unwrap();
                if char == ' ' || index >= char_line.len()-1 {
                    break;
                }else{
                    digit_line.push(char);
                    index+=1;
                }
            }
            println!("{:?}", digit_line);
            form.digits.push(digit_line);
        }
    }

    let result = transform_to_human(cephalopod_forms);
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
    let formulas = read_as_cephalopod(_input);
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
        assert_eq!(result.to_string(), "3263827");
    }
}
