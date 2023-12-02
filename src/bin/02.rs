const BLUE_CUBES: u8 = 14;
const RED_CUBES: u8 = 12;
const GREEN_CUBES: u8 = 13;

pub fn part_one(input: &str) -> Option<u32> {
    let games = input.lines().into_iter().filter_map(|line| {
        let mut split = line.split(":");
        let id = split.next().unwrap().trim_start_matches("Game ").parse::<u8>().unwrap();
        let valid = split.next().unwrap().split(";").all(|color_set| {
            let mut blue = 0;
            let mut red = 0;
            let mut green = 0;

            color_set.split(",").for_each(|color| {
                let mut color = color.trim().split(" ");
                let amount = color.next().unwrap().parse::<u8>().unwrap();
                let color = color.next().unwrap();
                match color {
                    "blue" => blue += amount,
                    "red" => red += amount,
                    "green" => green += amount,
                    _ => ()
                }
            });
            blue > BLUE_CUBES || red > RED_CUBES || green > GREEN_CUBES
        });
        Some(id as u32).filter(|_| valid)
    }).sum::<u32>();
    Some(games)
}

fn max_value(numbers: &Vec<u8>) -> u16 {
    *numbers.iter().max().unwrap_or(&0) as u16
}

pub fn part_two(input: &str) -> Option<u16> {
    let games: u16 = input.lines().into_iter().map(|line| {
        let colors = &line.split(":").last().unwrap();
        let mut game_blue: Vec<u8> = Vec::with_capacity(2);
        let mut game_red: Vec<u8> = Vec::with_capacity(2);
        let mut game_green: Vec<u8> = Vec::with_capacity(2);

        colors.split(";").for_each(|color_set| {
            let mut blue: u8 = 0;
            let mut red: u8 = 0;
            let mut green: u8 = 0;

            color_set.split(",").for_each(|color| {
                let mut color = color.trim().split(" ");
                let amount = &color.next().unwrap().parse::<u8>().unwrap_or(0);
                match color.next().unwrap() {
                    "blue" => blue += amount,
                    "red" => red += amount,
                    "green" => green += amount,
                    _ => ()
                }
            });

            game_blue.push(blue);
            game_red.push(red);
            game_green.push(green);
        });
        max_value(&game_blue)*max_value(&game_red)*max_value(&game_green)
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
