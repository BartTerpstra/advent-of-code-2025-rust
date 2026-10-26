use std::collections::HashMap;
use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 11)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 11 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn as_graph<'a>(file: &'a str, head:&str ) -> Graph<'a>{
    let lines = file.trim().lines();
    let mut results:Graph<'a> = HashMap::with_capacity(650);
    for line in lines {
        if line == "" {break;}

        let name = &line[0..3];
        let rest = &line[5..];
        let connected:Vec<&str> = rest.split(' ').collect();

        results.insert(name, connected);
    }
    results.iter().for_each(|x|->(){println!("{},{:?}", x.0,x.1);});

    results
}

fn burrow(graph: &Graph,position:&str, depth: usize, branch_count:usize) -> usize{
    if position.eq("out") {return branch_count;}
    if depth > 999{return 0;}

    let newdepth = depth+1;
    let mut newbranches = branch_count;

    let branches = graph.get(position).unwrap();
    newbranches +=branches.len()-1;
    for branch in branches {
        let val = burrow(graph,branch,newdepth,newbranches);
        if val > newbranches {
            newbranches = val;
        }
    }

    newbranches
}

fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 1
    //find all paths in a directed graph.
    let graph = as_graph(_input, "you");
    let val = burrow(&graph, "you", 0,1);


    Ok(val)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 2
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "5");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "0");
    }
}
