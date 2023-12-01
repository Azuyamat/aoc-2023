use std::ops::{Add};
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"\d").unwrap();
    let global_sum: u32 = input.lines().map(|line| {
        if line.is_empty() { return 0; }
        let mut local_nums: Vec<u32> = re.find_iter(line).map(|mat| mat.as_str().parse::<u32>().unwrap()).collect();
        let last = local_nums.pop().unwrap();
        let first = *local_nums.first().unwrap_or(&last);
        let local_sum = last * 10 + first;
        local_sum
    }).sum::<u32>();
    Some(global_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut global_sum = 0;
    for line in input.lines() {
        let re_first = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
        let re_second = Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
        let first = re_first.captures(line).unwrap().get(1).unwrap().as_str();
        let last = re_second.captures(line).unwrap().get(1).unwrap().as_str();
        let first = first.parse::<i32>().unwrap_or_else(|_| parse_num(first));
        let last = last.parse::<i32>().unwrap_or_else(|_| parse_num(last));
        let local_sum = first.to_string().add(last.to_string().as_str()).parse::<u32>().unwrap();
        global_sum+=local_sum;
    }

    Some(global_sum)
}

fn parse_num(num: &str) -> i32 {
    match num {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
