use std::net::UdpSocket;
use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 10)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 10 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

type BinaryArray = Vec<bool>;

//temp, probs, found by measuring.
const maxButtons: usize = 13;

#[derive(Debug)]
struct Machine{
    lights:Lights,
    buttons:Vec<Button>,
}

#[derive(Debug)]
struct Lights{
    target: BinaryArray
}
#[derive(Debug)]
struct Button{
    affects: BinaryArray
}

fn xor(left:bool, right: bool) -> bool{
    left == true && right == false || right == true && left == false
}
fn xor_array(left: &BinaryArray, right: &BinaryArray) -> BinaryArray{
    //assert equal length.
    let mut result = vec![false;left.len()];
    for i in 0..result.len(){
        result[i] = xor(left[i], right[i]);
    }
    result
}

//converts character that represents index of light to usize.
//there are no lightboxes with more than 10 lights.
fn char_as_usize(c: char) -> usize {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => 0
    }
}
fn as_machines(file:&str)->Vec<Machine>{
    let mut result = vec![];
    let lines = file.trim().lines();
    for line in lines {
        let mut words = line.split_whitespace();
        let target_lights_str = words.next().unwrap();
        let target_lights:BinaryArray = target_lights_str[1..target_lights_str.len()-1].chars().map(|x|->bool{x=='#'}).collect();

        let mut buttons = vec![];
        let mut joltage = "";
        while(true) {
            let next_word = words.next().unwrap();
            if (next_word.chars().next().unwrap() == '{'){
                joltage = next_word;
                break;
            }else{
                let inner_button = &next_word[1..next_word.len()-1];
                let affected: Vec<usize> = inner_button.split(",").map(|x| -> (usize) {char_as_usize(x.chars().next().unwrap())}).collect();

                let mut affects = vec![false; target_lights.len()];

                for index in affected {
                    affects[index] = true;
                }

                buttons.push(Button {affects})
            }
        }

        let lights = Lights{target:target_lights};
        result.push(Machine {lights, buttons})
    }
    result
}

fn as_binaryarray(i:u8, noButtons:usize)->BinaryArray{
    let mut results = vec![false;noButtons];
    //root is how many digits to place.
    //the count mod noButtons is where to place them?
    //well, kind of, but not really.
    //the sequence is very similar to https://oeis.org/A359941


    results
}

fn as_mod_number(i:u8, modulus:u8, maxDigits:usize) -> Vec<usize>{
    let mut results =  Vec::with_capacity(maxDigits);
    let mut index = 0;
    let mut rem = i % modulus;
    let mut upnext = i / modulus;

    results
}
fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    let mut machines = as_machines(_input);

    println!("started");
    //read and xor values until you have found a working combination.
    //very nice runtime properties of n+n-1+n-2, etc.
    //2026: god, i'm pretty clever. he realised it is a finite state. every button pressed twice is equivalent to not being pressed at all.
    //13 buttons is the max, so 2^13 combinations, is 8192.

    //idea: iterate over 0..2^buttons.len().
    //transform i into binary. every 1 is which buttons to combine.
    //combine, check against target.
    //repeat.

    //todo convert to working with binary types instead?

    let mut mostbuttons = 0;
    for mut machine in machines {
        if machine.buttons.len() > mostbuttons {
            mostbuttons = machine.buttons.len();
        }
    }

    //for every machine, find the lowest amount of button presses required to correctly configure them.
    //a machine is current lights, target lights, list of buttons, joltage.
    //buttons have a list of indexes of lights it flips when pressed.
    //lights are 0 initialised.

    //ways to get least button path (might be relevant for second excercise:
    //* breadth first brute-force:
    //given all current states, try every button on every state. if any results are the target, the shortest was found. if no results are target, create a new node for every results instead, goto 1.

    //* reverse exploration:
    //since the buttons are cumulative, they can be pressed in any order.
    //since certain lights are on, certain buttons MUST be in the combination.

    //check which buttons can affect the buttons we want on.
    //make a branch for each.
    //then, explore which other buttons can either turn on the remaining lights or turn off the wrong lights.
    //create a branch for each.
    //at any point, if a node matches target, stop.

    //a button might be required but have poor "fitness" in the above metric, it will at some point be explored anyway, as it will turn off buttons that are on for the wrong reason.
    //potential optim idea: buttons with higher fitness are allowed to look deeper.
    Ok(mostbuttons)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 2
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        //assert_eq!(result.to_string(), "0");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "0");
    }
}
