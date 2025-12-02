use crate::utils;
use anyhow::Result;
use prime_factorization::Factorization;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 2)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 2 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}



#[derive(Debug)]
struct Range{
    lower: u64,
    upper: u64
}
fn as_range_list(file: &str)->Vec<Range> {
    //assert no invalid data
    let mut ranges = vec![];
    for range in file.trim().split(',') {
        let mut parts = range.split('-');
        println!("{}", range);

        let lower = parts.next().unwrap().parse::<u64>().unwrap();
        let upper = parts.next().unwrap().parse::<u64>().unwrap();
        ranges.push(Range{lower, upper});
    }
    return ranges
}

fn check_equal_digits(i: u64)->u64 {
    //we love typing
    match i {
        0..=9 => 0,
        11 | 22 | 33 | 44 | 55 | 66 | 77 | 88 | 99 => i,
        111 | 222 | 333 | 444 | 555 | 666 | 777 | 888 | 999 => i,
        1111 | 2222 | 3333 | 4444 | 5555 | 6666 | 7777 | 8888 | 9999 => i,
        11111 | 22222 | 33333 | 44444 | 55555 | 66666 | 77777 | 88888 | 99999 => i,
        111111 | 222222 | 333333 | 444444 | 555555 | 666666 | 777777 | 888888 | 999999 => i,
        1111111 | 2222222 | 3333333 | 4444444 | 5555555 | 6666666 | 7777777 | 8888888 | 9999999 => i,
        11111111 | 22222222 | 33333333 | 44444444 | 55555555 | 66666666 | 77777777 | 88888888 | 99999999 => i,
        111111111 | 222222222 | 333333333 | 444444444 | 555555555 | 666666666 | 777777777 | 888888888 | 999999999 => i,
        1111111111 | 2222222222 | 3333333333 | 4444444444 | 5555555555 | 6666666666 | 7777777777 | 8888888888 | 9999999999 => i,
        _ => 0,
    }
}

//this is secretly check four, but don't mind it
fn check_two (i:u64) -> u64{
    let first_two = i/100; //remove least 2 significant digits
    let last_two = i - first_two*100; //remove 2 most significant digits
    if (first_two == last_two){
        return i;
    }
    0
}

fn check_three (i:u64) -> u64{
    let first_three = i/1000; //remove least 3 significant digits
    let last_three = i - first_three*1000; //remove 3 most significant digits
    if (first_three == last_three){
        return i;
    }
    0
}
fn check_four(i: u64) -> u64{
    //two and four
    let l = i - (i /100)*100;
    let r = i/100;
    if  l == r{return i}

    0
}

fn check_six(i:u64) -> u64{
    let l = i - (i /1000)*1000;
    let r = i/1000;
    if  l == r{return i}

    0
}
fn check_eight(i:u64) -> u64{
    let l = i - (i /10000)*10000;
    let r = i/10000;
    if  l == r{return i}

    0
}
fn check_nine(i:u64) -> u64{
    //check three(111(111)111)
    let l = check_three(i - i /1000);
    let r = check_three(i/1000);
    if  (l!= 0 && r != 0 && l == r){return i}
    //check nine
    //redundant
    0
}
fn check_ten(i:u64) -> u64{
    // //check 2
    // let five = i/100000000; //first two digits
    // let four = (i - (five * 100000000)) / 1000000;
    // let three = (i - (four * 1000000)- (five * 100000000)) / 10000;
    // let two = (i - (three * 10000)- (four * 1000000)- (five * 100000000)) / 100;
    // let one = (i - (two * 100)- (three * 10000)- (four * 1000000)- (five * 100000000)) / 1;
    //
    // //commutative
    // if five==one && one ==two && two == three && four == three{return i}

    //check 5
    let left = i - (i /100000)*100000;
    let right = i/100000;

    if (left == right) {return i;}

    //check 10
    //redundant
    0
}


fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    //assert smaller than 11 digits, < u64
    let ranges = as_range_list(_input);

    //split every range that crosses 10^n such that every range has 1 uniform digit length
    //sort
    // collapse? are we expecting overlapping ranges? (+count collapses for fun).
    let mut sum:u128 = 0;
    for range in &ranges {
        for i in range.lower..=range.upper {
            let addition = match i {
                0..=9 => 0,
                11 | 22 | 33 | 44 | 55 | 66 | 77 | 88 | 99 => i,
                100..=999 => 0,
                1000..=9999 => check_four(i),
                10000..=99999 => 0,
                100000..=999999 => check_six(i),
                1000000..=9999999 => 0,
                10000000..=99999999 => check_eight(i),
                100000000..=999999999 => 0,
                1000000000..=9999999999 => check_ten(i),
                _ => 0,
            };
            if addition != 0 {
                println!("addition: {}", addition);
            }

            sum += addition as u128;
        }
    }

    //only twice :)
    // //option a: brute force
    // for range in &ranges {
    //     for i in range.lower..=range.upper {
    //         let addition = match i {
    //             0        ..=9 => 0,
    //             11|22|33|44|55|66|77|88|99 => i,
    //             111|222|333|444|555|666|777|888|999 => i,
    //             1010     ..=9999 => check_four(i),
    //             11111|22222|33333|44444|55555|66666|77777|88888|99999 => i,
    //             100000   ..=999999 => check_six(i),
    //             1111111|2222222|3333333|4444444|5555555|6666666|7777777|8888888|9999999 => i,
    //             10000000  ..=99999999 => check_eight(i),
    //             100000000 ..=999999999 => check_nine(i),
    //             1000000000..=9999999999 => check_ten(i),
    //             _ => 0,
    //         };
    //         if addition !=0 {
    //             println!("addition: {}", addition);
    //         }
    //
    //         sum+=addition as u128;


    //option b: generators and cutters
    //divisors of digit count are the target repetition lengths
    //i.e. 10 has 2,5 and 1, not 3.
    //the n significant digits repeated, where n is a repetition length, can be the bounds
    //for every range length 1..=10
        //get prime divors
        //for every range
            //for every prime divisor
                //generate list given bounds and primacy
                //sum+=list.sum()

    //optim a: generate compiletime table of all combinations

    //note, this outline is wrong. we want to segment range list into 10 pieces and go by it piece by piece
    // for x in 1..=10u32 {
    //     let divisors = Factorization::run(x);
    //     let mut inter = divisors.factors;
    //     inter.push(1);
    //     let to_check = inter;
    //     for range in &ranges {
    //         for x in &to_check {
    //             let found_values = vec![0u64];
    //             sum += found_values.iter().map(|x|*x as u128).sum::<u128>();
    //         }
    //     }
    // }
    Ok(sum.to_string())
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 2
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";


    #[test]
    fn parser() {
        let result = as_range_list(EXAMPLE);
        println!("{:?}",result)
    }

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        //too high
        assert_eq!(result.to_string(), "1227775554");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "0");
    }
}
