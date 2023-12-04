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
    let re = regex::Regex::new(r"\d+").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let matches: Vec<(usize, Captures)> = lines.iter().enumerate().map(|(i, line)| {
        let captures = re.captures(line).unwrap();
        (i, captures)
    }).collect::<Vec<(usize, Captures)>>();

    for (i, line) in lines.iter().enumerate() {
        let current_line_matches = matches.iter().filter(|(j, _)| *j == i);
        for matches in current_line_matches {
            let range = matches.1.get(0).unwrap().range();
            let start = range.start;
            let end = range.end;
            let points = get_box(start, end, i, lines.len(), true);
            let adjacent_matches = matches.1.iter().filter(|capture| {
                let range = capture.range();
                let start = range.start;
                let end = range.end;
                points.iter().any(|(x, y)| {
                    let line = lines.get(*y).unwrap();
                    let c = line.chars().nth(*x).unwrap();
                    is_part(c)
                })
            });
        }


    }
    println!("{:?}", matches);
    Some(0)
}

// Function to get all points around a point of a certain horizontal length
fn get_box(start: usize, end: usize, current_y: usize, max_y: usize, only_current_line: bool) ->
                                                                                              Vec<
    (usize, usize)> {
    let start = start.saturating_sub(1);
    let end = end.saturating_add(1);

    let mut start_y = max(0, current_y.saturating_sub(1));
    if only_current_line { start_y = current_y; }
    let end_y = min(max_y, current_y.saturating_add(1));

    let mut points: Vec<(usize, usize)> = Vec::new();
    for i in start..end { // X axis
        for j in start_y..end_y { // Y axis
            points.push((i, j));
        }
    }
    points
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
