use std::cmp::{max, min};
use regex::Captures;

const fn is_part(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = regex::Regex::new(r"\d+").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut global_sum: u32 = 0;
    for (i, line) in lines.iter().enumerate() {
        let i = i as isize;
        for capture in re.captures_iter(&line).filter_map(|capture| {
            // Get whole match as u32
            let number = capture.get(0).unwrap().as_str().parse::<u32>().unwrap();
            let line_above = lines.get((i - 1) as usize);
            let line_below = lines.get((i + 1) as usize);
            let capture_range = capture.get(0).unwrap().range();
            // Extend range by one each side (unless it goes above or below line length)
            let start = capture_range.start.checked_sub(1).unwrap_or(0);
            let end = capture_range.end.checked_add(1).unwrap_or(line.len());
            let capture_range = start..end;
            // Get chars in range `capture_range` from current line
            let line_contains_part: bool = line.char_indices().any(|(i, c)| {
                capture_range.contains(&i) && is_part(c)
            });
            if line_contains_part {
                return Some(number);
            }
            if line_above.is_some() {
                let line_above = line_above.unwrap();
                let line_above_contains_part: bool = line_above.char_indices().any(|(i, c)| {
                    capture_range.contains(&i) && is_part(c)
                });
                if line_above_contains_part {
                    return Some(number);
                }
            }
            if line_below.is_some() {
                let line_below = line_below.unwrap();
                // Get chars in range `capture_range` from line below
                let line_below_contains_part: bool = line_below.char_indices().any(|(i, c)| {
                    capture_range.contains(&i) && is_part(c)
                });
                if line_below_contains_part {
                    return Some(number);
                }
            }
            None
        }) {
            global_sum+=capture;
        }
    }
    Some(global_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let gears: Vec<(usize, Vec<(usize)>)> = lines.iter().enumerate().filter_map(|(index, line)| {
        let local_gears: Vec<usize> = line.chars().enumerate().filter_map(|(index, c)| { if c == '*' { Some(index) } else { None } }).collect::<Vec<usize>>();
        return if local_gears.is_empty() { None } else { Some((index, local_gears)) };
    }).collect::<Vec<(usize, Vec<(usize)>)>>();
    let mut global_sum = 0;
    for gear_line in gears {
        let y = gear_line.0;
        let local_gears = gear_line.1;
        for gear in local_gears { // Gear is x pos
            let surrounding_numbers = get_surrounding_numbers(gear, y, &lines);
            if surrounding_numbers.len() != 2 { continue; }
            let left = surrounding_numbers.get(0).unwrap();
            let right = surrounding_numbers.get(1).unwrap();
            let local_add = left*right;
            global_sum+=local_add;
        }
    }
    Some(global_sum)
}

fn get_surrounding_numbers(x: usize, y: usize, lines: &Vec<&str>) -> Vec<u32> {
    let re = regex::Regex::new(r"\d+").unwrap();

    let min_y = max(0, y.saturating_sub(1));
    let max_y = min(lines.len(), y.saturating_add(2));
    let x_vec = vec!(x.saturating_sub(1), x, x.saturating_add(1));

    let surrounding_numbers = (min_y..max_y).map(|y| {
        let line = lines.get(y).unwrap();
        let captures = re.captures_iter(&line).collect::<Vec<Captures>>();
        let numbers: Vec<u32> = captures.iter().filter_map(|capture| {
            let capture = capture.get(0).unwrap();
            let range = capture.range();
            // Check if range contains x from x_vec
            let valid = x_vec.iter().any(|x| range.contains(x));
            if !valid { return None; }
            Some(capture.as_str().parse::<u32>().unwrap())
        }).collect::<Vec<u32>>();
        numbers
    }).flatten().collect::<Vec<u32>>();
    surrounding_numbers
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    // aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
