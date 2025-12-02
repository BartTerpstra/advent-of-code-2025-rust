use crate::utils;
use anyhow::Result;

//todo optim a: generate compiletime table of all combinations
//todo sort and split every range that crosses 10^n such that every range has 1 uniform digit length
//there is something fancy you can do where you construct every combination using just the upper and lower bound where you don't iterate every value between
//but where instead you construct every digit through addition and multiplication
//which should be faster than going over every digit and doing multiple divisions.
//the code as it is, is faster than I expected, which could either be because of a small data set or the compiler saving me from myself
//use prime_factorization::Factorization;

//I went with a brute force approach that was quite fragile at first because I didn't feel good today
//and didn't get enough sleep

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

fn is_equal_half(i:u64, digits:u64) ->u64{
    let divisor = 10_u64.pow(digits as u32);

    //the following block exploits the fact that integer division rounds down,
    //allowing you to erase digits you push past the comma
    let first_half = i/divisor;
    let last_half = i - (i/divisor) *divisor;
    if (first_half == last_half){
        return i;
    }
    0
}

fn check_six_digits(i:u64) -> u64{
    //division by 3
    let first = i/10000;
    let last = i - (i /100)*100;
    let middle = (i - first *10000 - last)/100;
    if first == middle && middle == last {return i}

    //division by two
    is_equal_half(i, 6/2)
}
fn check_eight(i:u64) -> u64{
    //division by 2 and four
    //all divisions by four also create a division by two
    is_equal_half(i, 8/2)
}
fn check_nine(i:u64) -> u64{
    //division by 3
    let first = i/1000000;
    let last = i - (i /1000)*1000;
    let middle = (i - first *1000000 - last)/1000;

    if first == middle && middle == last {return i}

    0
}
fn check_ten(i:u64) -> u64{
    // division by five
    let five = i/100000000; //first two digits
    let four = (i - (five * 100000000)) / 1000000;
    let three = (i - (four * 1000000)- (five * 100000000)) / 10000;
    let two = (i - (three * 10000)- (four * 1000000)- (five * 100000000)) / 100;
    let one = (i - (two * 100)- (three * 10000)- (four * 1000000)- (five * 100000000)) / 1;

    //commutative
    if five==one && one ==two && two == three && four == three{return i}

    //division by 2
    is_equal_half(i, 5)
}


fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    //assert smaller than 11 digits, < u64
    //assert no overlapping ranges
    let ranges = as_range_list(_input);

    let mut sum:u128 = 0;
    for range in &ranges {
        for i in range.lower..=range.upper {
            let addition = match i {
                0..=9 => 0,
                11 | 22 | 33 | 44 | 55 | 66 | 77 | 88 | 99 => i,
                100..=999 => 0,
                1000..=9999 => is_equal_half(i, 2),
                10000..=99999 => 0,
                100000..=999999 => is_equal_half(i, 3),
                1000000..=9999999 => 0,
                10000000..=99999999 => is_equal_half(i, 4),
                100000000..=999999999 => 0,
                1000000000..=9999999999 => is_equal_half(i, 5),
                _ => 0,
            };

            sum += addition as u128;
        }
    }

    Ok(sum.to_string())
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    let ranges = as_range_list(_input);
    let mut sum:u128 = 0;
    for range in &ranges {
        for i in range.lower..=range.upper {
            let addition = match i {
                0..=9 => 0,
                11 | 22 | 33 | 44 | 55 | 66 | 77 | 88 | 99 => i,
                111 | 222 | 333 | 444 | 555 | 666 | 777 | 888 | 999 => i,
                1010..=9999 => is_equal_half(i, 4/2),
                11111 | 22222 | 33333 | 44444 | 55555 | 66666 | 77777 | 88888 | 99999 => i,
                100000..=999999 => check_six_digits(i),
                1111111 | 2222222 | 3333333 | 4444444 | 5555555 | 6666666 | 7777777 | 8888888 | 9999999 => i,
                10000000..=99999999 => check_eight(i),
                100000000..=999999999 => check_nine(i),
                1000000000..=9999999999 => check_ten(i),
                _ => 0,
            };

            sum += addition as u128;
        }
    }
    Ok(sum.to_string())
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
        assert_eq!(result.to_string(), "1227775554");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "4174379265");
    }
}
