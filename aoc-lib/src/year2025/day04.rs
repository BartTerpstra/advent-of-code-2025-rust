use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 4)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 4 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

type Shopfloor = Vec<Vec<bool>>;
struct Coordinate {height:usize, width:usize}

fn as_shopfloor(input: &str) -> Shopfloor{
    let mut results = vec![];
    let mut lines = input.trim().lines().peekable();
    let mut width = lines.peek().unwrap().len();
    width+=2; // we are adding an edge to the entire shop
    results.push(vec![false;width]); //empty row
    for line in lines {
        let line_chars = line.chars();
        let mut shop_line = vec![false;width];
        let mut index = 1;
        for char in line_chars {
            if char == '@' {
                shop_line[index] = true;
            }
            index+=1;
        }
        results.push(shop_line)
    }
    results.push(vec![false;width]); //empty row

    results
}

fn to_num(boolean:bool)->u8{
    if boolean {
        return 1
    }
    0
}

fn getAvailable(floor: &Shopfloor, coordinate:Coordinate) -> u8{
    //assert coordinates are always atleast 1 in from the edge
    let mut sum = 0;
    sum += to_num(floor[coordinate.height-1][coordinate.width-1]);
    sum += to_num(floor[coordinate.height-1][coordinate.width]);
    sum += to_num(floor[coordinate.height-1][coordinate.width+1]);

    sum += to_num(floor[coordinate.height][coordinate.width-1]);
    // sum += to_num(floor[coordinate.height][coordinate.width]);
    sum += to_num(floor[coordinate.height][coordinate.width+1]);

    sum += to_num(floor[coordinate.height+1][coordinate.width-1]);
    sum += to_num(floor[coordinate.height+1][coordinate.width]);
    sum += to_num(floor[coordinate.height+1][coordinate.width+1]);

    sum
}
fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    let floor  = as_shopfloor(_input);
    let mut total = 0;
    //-1 because we are skipping the far edge
    let max_height = floor.len()-1;
    let max_width = floor[0].len()-1;
    for height in 1..max_height {
        for width in 1..max_width{
            if getAvailable(&floor, Coordinate{height, width}) < 4{
                total+=1
            }
        }
    }

    Ok(total)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 2
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_parse() {
        let result = as_shopfloor(EXAMPLE);
        println!("{:?}", result)
    }

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "13");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "0");
    }
}
