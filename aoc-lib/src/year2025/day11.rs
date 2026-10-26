use std::collections::{HashMap, LinkedList};
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

fn as_graph<'a>(file: &'a str) -> Graph<'a>{
    let lines = file.trim().lines();
    let mut results:Graph<'a> = HashMap::with_capacity(650);
    for line in lines {
        if line == "" {break;}

        let name = &line[0..3];
        let rest = &line[5..];
        let connected:Vec<&str> = rest.split(' ').collect();

        results.insert(name, connected);
    }
    results
}

fn burrow(graph: &Graph,position:&str, mut depth: usize, mut branch_count:usize) -> usize{
    if position.eq("out") {return branch_count;}
    if depth > 999{return 0;}

    depth +=1;
    let branches = graph.get(position).unwrap();
    branch_count +=branches.len()-1;
    for branch in branches {
        let val = burrow(graph,branch,depth,branch_count);
        if val > branch_count {
            branch_count = val;
        }
    }

    branch_count
}

fn burrow2(graph: &mut Graph, from:&str, to: &str, mut depth: usize) -> usize{
    if from.eq(to) {
        return 1
    }
    if from.eq("out"){
        return 0
    }
    if depth >= 999{return 0;}

    let node = graph.get(from);
    if node == None {return 0;}


    depth +=1;
    let branches = node.unwrap().clone();
    graph.remove(from);
    let mut sum = 0;

    for branch in branches {
        println!("currently at: {}-{}", from, branch);

        let val = burrow2(graph, branch,to,depth);
        sum+=val;
    }

    sum
}

fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    //find all paths in a directed graph.
    let graph = as_graph(_input);
    let val = burrow(&graph, "you", 0,1);
    Ok(val)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    //find all paths in a directed graph between srv and out that pass through dac and fft
    //brute force doesn't work.

    //dac runs to fft, fft probably does not run to dac.
    //8267 possible paths dac -> fft
    //svr to dac too large to brute
    //fft to out too large to brute

    //an option could be to reverse the graph, but that should have similar complexity unless a graph was designed to be faster in reverse.
    let mother = as_graph(_input);
    let mut graph1 = mother.clone();
    let mut graph2 = mother.clone();
    let mut graph3 = mother.clone();
    println!("start first");
    let first = burrow2(&mut graph1, "svr", "fft", 0);
    println!("start second");
    let middle = burrow2(&mut graph2, "fft", "dac", 0);
    println!("start third");
    let last = burrow2(&mut graph3, "dac", "out", 0);

    let answer1 = first*middle*last;

    let mut graph1 = mother.clone();
    let mut graph2 = mother.clone();
    let mut graph3 = mother.clone();
    let mut graph4 = mother.clone();
    println!("start first");
    let first = burrow2(&mut graph1, "fft", "dac", 0);
    println!("start second");
    let middle = burrow2(&mut graph2, "dac", "svr", 0);
    println!("start third");
    let last = burrow2(&mut graph3, "fft", "svr", 0);

    let last2 = burrow2(&mut graph4, "fft", "out", 0);

    let answer2 = answer1*2*1*1*last2;

    //not 54
    //not 1944
    //148806
    Ok(answer2)
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

    const EXAMPLE2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "5");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE2).unwrap();
        assert_eq!(result.to_string(), "2");
    }
}
