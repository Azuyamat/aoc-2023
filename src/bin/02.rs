use std::cmp::max;

const BLUE_CUBES: u8 = 14;
const RED_CUBES: u8 = 12;
const GREEN_CUBES: u8 = 13;

pub fn part_one(input: &str) -> Option<u16> {
    Some(input.lines().into_iter().enumerate().filter_map(|(id, line)| {
        let mut parts = line.split(":").last().unwrap().split(";");
        let valid = parts.all(|color_set| {
            let mut blue = 0;
            let mut red = 0;
            let mut green = 0;

            color_set.split(",").for_each(|color| {
                let mut color = color.trim().split(' ');
                let amount = color.next().unwrap().parse::<u8>().unwrap();
                let color = color.next().unwrap();
                match color.chars().next().unwrap() {
                    'b' => blue += amount,
                    'r' => red += amount,
                    'g' => green += amount,
                    _ => ()
                }
            });
            !(blue > BLUE_CUBES || red > RED_CUBES || green > GREEN_CUBES)
        });
        Some((id+1) as u16).filter(|_| valid)
    }).sum::<u16>())
}

pub fn part_two(input: &str) -> Option<u16> {
    let games: u16 = input.lines().into_iter().map(|line| {
        let colors = &line.split(":").last().unwrap();
        let mut game_blue: u16 = 0;
        let mut game_red: u16 = 0;
        let mut game_green: u16 = 0;

        colors.split(";").for_each(|color_set| {
            let mut blue: u16 = 0;
            let mut red: u16 = 0;
            let mut green: u16 = 0;

            color_set.split(",").for_each(|color| {
                let mut color = color.trim().split(" ");
                let amount = &color.next().unwrap().parse::<u16>().unwrap_or(0);
                match color.next().unwrap() {
                    "blue" => blue += amount,
                    "red" => red += amount,
                    "green" => green += amount,
                    _ => ()
                }
            });

            game_blue = max(game_blue, blue);
            game_red = max(game_red, red);
            game_green = max(game_green, green);
        });
        &game_blue*&game_red*&game_green
    }).sum::<u16>();
    Some(games)
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(2169));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(60948));
    }
}
