use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 8)?;

    let part1 = solve_part1(&input, 1000)?;
    let part2 = solve_part2(&input)?;

    println!("Day 8 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[derive(Clone)]
struct Hub {
    x: i64,
    y: i64,
    z: i64,
    id: u128
}

struct Link{
    to:Hub,
    from:Hub,
    distance:u64
}

struct Network{
    links: Vec<Link>,
    id:u128
}
fn abs_diff(a:i64,b:i64) -> i64{
    (a-b).abs()
}
fn get_distance(a:&Hub, b:&Hub) ->u64{
    //sum of difference between 3 axis.
    let x = abs_diff(a.x,b.x);
    let y = abs_diff(a.y,b.y);
    let z = abs_diff(a.z,b.z);

    let distance = (x+y+z) as u64;//todo change this

    distance
}

fn solve_part1(_input: &str, mut connections_to_make:u32) -> Result<impl std::fmt::Display> {
    //really, this is like a naive solution to the 3 dimensional traveling salesman problem
    //combined with dijkstra

    //let's have virtual networks.
    //start with a list of points with network IDs equal to their index
    //for every point against every point not checked (n^2)
    let hubs:Vec<Hub> = vec![];

    //calculate distance
    //store in list with ref L, ref R, distance.
    //sort.
    let mut links:Vec<Link> = vec![];
    for index in 0..hubs.len(){
        for inner in index..hubs.len(){
            let distance = get_distance(
                &hubs[index],
                &hubs[inner]
            );
            let l = Link{
                to: (&hubs[index]).clone(),
                from: hubs[inner].clone(),
                distance};
            links.push(l);
        }
    }

    let mut networks:Vec<Network> = vec![];
    while (connections_to_make > 0){
    }
    //for every item
    //if L.get_network == R.get_network continue
    //else combine_networks(l.get_network, r.get_network)
    Ok(0)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 2
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE, 40).unwrap();
        assert_eq!(result.to_string(), "40");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "0");
    }
}
