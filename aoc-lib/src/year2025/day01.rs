use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 1)?;
    let instructions = to_instructions(&input);

    let part1 = solve_part1(&instructions)?;
    let part2 = solve_part2(&instructions)?;

    println!("Day 1 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

struct Machine{
    position: i32,
    zerocount: u32,
}

fn new()->Machine{
    Machine{position:50, zerocount:0}
}

struct Instruction {
    is_left: bool,
    value:u8,
    zerocount_bonus:u8,
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
        let zerocount_bonus:u8 = (slice / 100) as u8;
        let value:u8 = (slice % 100) as u8;
        instructions.push(Instruction { is_left: is_left(direction),value:value, zerocount_bonus:zerocount_bonus});
    }
    return instructions;
}
fn solve_part1(instructions: &Vec<Instruction>) -> Result<u32> {
    let mut machine = new();
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

    Ok(machine.zerocount)
}

fn solve_part2(instructions: &Vec<Instruction>) -> Result<u32> {
    let mut machine = new();
    let mut was_zero:bool = false;
    for instruction in instructions {
        if instruction.is_left {
            machine.position -= instruction.value as i32;
            if machine.position < 0 {
                machine.position = 100+machine.position;
                if !was_zero {
                    machine.zerocount += 1;
                }
            }else{
                if machine.position == 0 {
                    was_zero = true;
                    machine.zerocount += 1;
                }
            }
        }else{
            machine.position += instruction.value as i32;
            if machine.position > 99 {
                machine.position -= 100;
                if !was_zero {
                    machine.zerocount += 1;
                }
            }else{
                if machine.position == 0 {
                    machine.zerocount += 1;
                }
            }
        }
        was_zero = machine.position == 0;
    }

    let bonus =  instructions.iter().map(|x|->u32{x.zerocount_bonus as u32}).sum::<u32>();
    Ok(machine.zerocount +bonus)
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
        let instructions = to_instructions(EXAMPLE);
        let result = solve_part1(&instructions).unwrap();
        assert_eq!(result.to_string(), "3");
    }

    #[test]
    fn test_part2() {
        let instructions = to_instructions(EXAMPLE);
        let result = solve_part2(&instructions).unwrap();
        assert_eq!(result.to_string(), "6");
    }
}
