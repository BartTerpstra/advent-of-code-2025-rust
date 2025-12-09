use std::fmt;
use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 7)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 7 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[derive(PartialEq)]
#[derive(Clone)]
enum TachState {
    Empty,
    Beam,
    Splitter,
    Bug
}

impl fmt::Display for TachState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let answer = match self{
            TachState::Empty => ".",
            TachState::Beam => "|",
            TachState::Splitter => "^",
            TachState::Bug => "🐛"
        };
        write!(f, "{}", answer)
    }
}

fn as_tach_state(char: char) -> TachState{
    match char{
        '.' =>TachState::Empty ,
        '|' =>TachState::Beam ,
        'S' => TachState::Beam,
        '^' => TachState::Splitter ,
        _ => TachState::Bug
    }
}

fn as_sim_field(_input:&str) -> Vec<Vec<TachState>>{
    let mut result = vec![];
    let lines = _input.trim().lines();

    for line in lines {
        let mut sim_line = vec![];
        for char in line.chars() {
            sim_line.push(as_tach_state(char))
        }
        result.push(sim_line)
    }

    result
}




fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    //virtual would be too much work and error-prone
    //let's just run a full sim.
    //142^2, approx 20.000 elements, assuming u8 per elements, that's 20kbish ram, is fine.
    //time is linear or better, we're just doing 1 line at a time and there is no interference between operations.

    let mut sim = as_sim_field(_input);
    let mut split_count = 0;

    //assert no splitters are next to eachother ^^
    //such a file is considered ill-formed and safety mechanism will not be implemented.

    //for every line, starting with second and given look-back 1.
    //for every character in look-back, if |
    //check if index of line is empty, if empty, set |
    //                      is splitter, set neighbours to |
    for line_i in 1..sim.len(){
        let lookback = sim.get(line_i-1).unwrap().clone();
        let line = sim.get_mut(line_i).unwrap();

        for current_w_index in 0..lookback.len() {
            let above_state = lookback.get(current_w_index).unwrap();
            if *above_state == TachState::Beam {
                let below_state = line.get_mut(current_w_index).unwrap();
                match below_state {
                    TachState::Empty => {*below_state = TachState::Beam }
                    TachState::Splitter => {
                        //assert a splitter will not be placed on the edge of the sim
                        let l = line.get_mut(current_w_index +1).unwrap();
                        *l = TachState::Beam;
                        let r = line.get_mut(current_w_index -1).unwrap();
                        *r = TachState::Beam;

                        //DEBUG/RESULT: if setting because splitter, increment counter
                        split_count+=1;
                    }
                    _ => {}
                }
            }
        }

        for line in &sim {
            for state in line {
                print!("{}", state)
            }
            println!()
        }
        println!("------------------")
    }

    //return counter
    Ok(split_count)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 2
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "21");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "0");
    }
}
