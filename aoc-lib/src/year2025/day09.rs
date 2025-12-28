use std::io::Lines;
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

enum Direction{
    LEFT,
    UP,
    RIGHT,
    DOWN,
}

struct Line{
    to:Point,
    from:Point,
    direction: Direction
}

struct ClosedGraph {
    lines: Vec<Line>,
    inside_is_left_handed:bool // a closed graph has an inside and an outside, which can be determined by taking the handed ness of the first line segment from to.
}
fn area(a:&Point, b: &Point)->u64{
    let xdiff = (a.x-b.x).abs() as u64;
    let ydiff = (a.y-b.y).abs() as u64;
    (xdiff+1)*(ydiff+1)
}

fn as_points(_input:&str) -> Vec<Point>{
    let mut points: Vec<Point> = vec![];

    let lines = _input.trim().lines();
    for line in lines {
        let halves = line.split_once(",").unwrap();
        points.push(Point{x:halves.0.parse().unwrap() ,y:halves.1.parse().unwrap()})
    }

    points
}

fn as_closed_graph(points: &Vec<Point>)->ClosedGraph{
    //construct lines and keep track of total rotation, +/- 360 at the end will show handedness
    return ClosedGraph{ lines: vec![], inside_is_left_handed: false }
}
fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    //There is no floor neo!
    let points = as_points(_input);

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
    // I knew it was too good to be true
    // a floor is still too expensive
    //we're just doing the partial and see what happens.

    let points = as_points(_input);
    //clone, sort by x
    let graph = as_closed_graph(&points);
    //clone, filter vertical sort by x
    //clone, filter horizontal  sort by y

    let mut areas = vec![0_u64; points.len()*points.len()];

    for index in 0..points.len(){
        for inner in index..points.len(){
            //todo only store if points are inside in respect to each other
            let area = area(&points[index], &points[inner]);
            areas.push(area);
        }
    }
    //sort descending
    areas.sort();
    areas.reverse();

    //check if any points inside it, if so, discard it.
        //for sorted points. binary search indexes of outer line.
        //for every index between, check if y value is also between outer line.

    //check if any line crosses it, if so, discard it.
        //for every line sorted by x
        //binary search outer indexes.
        //for every index, check if spans y.

        //for every line sorted by y
        //binary search outer indexs.
        //for every index, check if spans x.

    //if passes, return.


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
