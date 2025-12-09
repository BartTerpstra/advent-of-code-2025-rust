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

fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    //virtual would be too much work and error-prone
    //let's just run a full sim.
    //142^2, approx 20.000 elements, assuming u8 per elements, that's 20kbish ram, is fine.
    //time is linear or better, we're just doing 1 line at a time and there is no interference between operations.


    //for file as string
    //trim
    //lines
    //per line, create sim line
    //per character, transform to 0 1, 2 (enum with display?)
    //append line to result
    //result

    //assert no splitters are next to eachother ^^
    //such a file is considered ill-formed and safety mechanism will not be implemented.

    //for every line, starting with second and given look-back 1.
    //for every character in look-back, if |
    //check if index of line is empty, if empty, set |
    //                      is splitter, set neighbours to |

    //DEBUG/RESULT: if setting because splitter, increment counter

    //return counter
    Ok(0)
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
