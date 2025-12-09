use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 9)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 9 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

//assert X-y in the standard computer graphics tradition. (origin is top left, y is height, x is width)
#[derive(Debug)]
struct Point{
    x: i64,
    y: i64
}

fn area(a:&Point, b: &Point)->u64{
    let xdiff = (a.x-b.x).abs() as u64;
    let ydiff = (a.y-b.y).abs() as u64;
    (xdiff+1)*(ydiff+1)
}
fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    //There is no floor neo!

    let mut points: Vec<Point> = vec![];

    let lines = _input.trim().lines();
    for line in lines {
        let halves = line.split_once(",").unwrap();
        points.push(Point{x:halves.0.parse()? ,y:halves.1.parse()?})
    }

    let mut max = 0;
    let mut maxpoints = (&Point{x:0,y:0},&Point{x:0,y:0});
    for index in 0..points.len(){
        for inner in index..points.len(){
            let area = area(&points[index], &points[inner]);
            if area > max {
                max = area;
                maxpoints = (&points[index], &points[inner])
            }
        }
    }
    println!("{:?}",maxpoints);
    Ok(max)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 2
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "50");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "0");
    }
}
