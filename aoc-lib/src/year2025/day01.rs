use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 1)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 1 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[derive(Debug)]
struct Machine{
    position: i32,
    zerocount: u32,
}
fn new()->Machine{
    Machine{position:50, zerocount:0}
}
#[derive(Debug)]
struct Instruction {
    is_left: bool,
    value:u8,
}

fn is_left(c:char) ->bool{
    c == 'L'
}

fn to_instructions(file:&str) ->Vec<Instruction>{
    let mut instructions: Vec<Instruction>= vec![];
    for line in file.trim().lines() {
        //assert len 2-3
        let direction: char = line.chars().nth(0).unwrap();
        //assert all values between 0 and 999 < 1024 = u32
        let slice: u32 = (&line[1..]).parse::<u32>().unwrap();
        let value:u8 = (slice % 100) as u8;
        instructions.push(Instruction { is_left: is_left(direction),value:value});
    }
    return instructions;
}
fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    let mut machine = new();
    let instructions = to_instructions(_input);
    for instruction in instructions {
        if instruction.is_left {
            machine.position -= instruction.value as i32;
            if machine.position < 0 {
                machine.position = 100+machine.position;
            }
        }else{
            machine.position += instruction.value as i32;
            if machine.position > 99 {
                machine.position -= 100
            }
        }
        if machine.position == 0 {
            machine.zerocount += 1;
        }


    }

    Ok(machine.zerocount.to_string())
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 2
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "3");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "6");
    }
}
