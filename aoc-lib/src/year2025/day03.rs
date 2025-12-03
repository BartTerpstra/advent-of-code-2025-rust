use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(2025, 3)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day 3 / Year 2025");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn as_banks(file: &str) -> Vec<Vec<char>>{
    //assert 100 digits > u128 == a problem
    //assert no invalid input
    let mut results:Vec<Vec<char>> = Vec::with_capacity(200);
    let lines = file.trim().lines();
    for x in lines {
        results.push(digits_of(x));
    }

    results
}
fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {

    let banks = as_banks(_input);
    let mut sum=0;
    for bank in banks {
        //assert highest is never allowed to be the last index (14)
        //assert indexes are not allowed to be equal
        let mut index_of_highest = 0;
        let mut index_of_second_highest;
        for i in 1..bank.len()-1{ //last index is not allowed!
            //assert the following absolute ordering for chars: 0<1<2<3<4<5<6<7<8<9
            if bank[index_of_highest] < bank[i]{
                index_of_highest = i;
                //small optim
                if bank[index_of_highest] == '9'{
                    break;
                }
            }
        }
        index_of_second_highest = index_of_highest+1;

        for i in index_of_second_highest..bank.len(){
            if bank[index_of_second_highest] < bank[i]{
                println!("updated");
                println!("{:?}", bank);
                println!("high: {}, curr: {}, new:{}", index_of_highest, index_of_second_highest, i);

                index_of_second_highest = i;
            }
        }
        let tensplace = bank[index_of_highest].to_digit(10).unwrap()*10;
        let onesplace = bank[index_of_second_highest].to_digit(10).unwrap();
        println!("value {}",bank[index_of_second_highest]);
        println!("index {}",index_of_second_highest);
        println!("interpreted as {}",onesplace);
        let charge = tensplace+onesplace;
        println!("charge: {}",charge);
        sum+=charge;
    }
    Ok(sum)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    let banks = as_banks(_input);
    let mut sum=0;
    let digits = 12;
    for bank in banks {
        let mut highest_indices = vec![0;digits];
        for digit in 0..digits {
            //assert highest is never allowed to be the last index (14)
            //assert indexes are not allowed to be equal
            let start = highest_indices[digit];
            let digits_needed_after = digits-(digit+1);
            //e.g. 100 - 11
            let end = bank.len() - digits_needed_after;

            for i in start..end{ //indexes before or after current not allowed
                //assert the following absolute ordering for chars: 0<1<2<3<4<5<6<7<8<9
                if bank[highest_indices[digit]] < bank[i] || (i==highest_indices[digit]){
                    highest_indices[digit] = i;
                    if bank[highest_indices[digit]] == '9'{
                        if(digit+1 != digits) {
                            highest_indices[digit + 1] = highest_indices[digit] + 1;
                        }
                        break;
                    }
                }
            }
            if(digit+1 != digits) {
                highest_indices[digit + 1] = highest_indices[digit] + 1;
            }
        }
        let mut inner:u64 = 0;
        highest_indices.reverse();
        //the first index should be multipled by 10^size, not the last.
        //assert max is summing 200 12 digit numbers < 50 bits < u64
        for x in 0..highest_indices.len(){
            //brother... why in the world am i casting this much? am i doing it wrong?
            inner += (bank[highest_indices[x]].to_digit(10).unwrap() as u64) * 10u64.pow(x as u32)
        }
        println!("sum{}", inner);
        sum+=inner;
    }

    Ok(sum.to_string())
}

fn digits_of(i:&str)->Vec<char> {
    let result = i.chars().collect::<Vec<char>>();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_parser() {
        let result = as_banks(EXAMPLE);
        println!("{:?}", result);
        // assert_eq!(result.to_string(), "357");
    }

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "357");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "3121910778619");
    }
}
