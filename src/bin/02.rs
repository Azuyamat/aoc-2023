#[derive(Debug)]
struct Game {
    id: u8,
    blue: u8,
    red: u8,
    green: u8,
    valid: bool
}

impl Game {
    fn new(id: u8) -> Self {
        Self {
            id,
            blue: 0,
            red: 0,
            green: 0,
            valid: true
        }
    }
}

const BLUE_CUBES: u8 = 14;
const RED_CUBES: u8 = 12;
const GREEN_CUBES: u8 = 13;

pub fn part_one(input: &str) -> Option<u32> {
    let games: Vec<Game> = input.lines().into_iter().map(|line| {
        let split = line.split(":").collect::<Vec<&str>>();
        let id = split[0].replace("Game ", "").parse::<u8>().unwrap();
        let colors = split[1];
        let mut game = Game::new(id);
        colors.split(";").for_each(|color_set| {
            let mut blue = 0;
            let mut red = 0;
            let mut green = 0;

            let color_set = color_set.split(",").collect::<Vec<&str>>();
            color_set.iter().for_each(|color| {
                let color = color.trim().split(" ").collect::<Vec<&str>>();
                let amount = color[0].parse::<u8>().unwrap();
                let color = color[1];
                match color {
                    "blue" => blue += amount,
                    "red" => red += amount,
                    "green" => green += amount,
                    _ => ()
                }
            });
            game.blue += blue;
            game.red += red;
            game.green += green;
            if blue > BLUE_CUBES || red > RED_CUBES || green > GREEN_CUBES { game.valid = false; }
        });
        return game;
    }).collect();
    let games = games.iter().filter(|game| game.valid == true).collect::<Vec<&Game>>();
    Some(games.iter().map(|game| game.id as u32).sum::<u32>())
}

#[derive(Debug)]
struct Game2 {
    blue: Vec<u8>,
    red: Vec<u8>,
    green: Vec<u8>,
}

fn max_value(numbers: &Vec<u8>) -> u16 {
    *numbers.iter().max().unwrap_or(&0) as u16
}

pub fn part_two(input: &str) -> Option<u16> {
    let games: u16 = input.lines().into_iter().map(|line| {
        let colors = &line.split(":").last().unwrap();
        let mut game = Game2 {
            blue: Vec::with_capacity(2),
            red: Vec::with_capacity(2),
            green: Vec::with_capacity(2)
        };

        colors.split(";").for_each(|color_set| {
            let mut blue: u8 = 0;
            let mut red: u8 = 0;
            let mut green: u8 = 0;

            color_set.split(",").for_each(|color| {
                let color = color.trim().split(" ").collect::<Vec<&str>>();
                let amount = &color[0].parse::<u8>().unwrap_or(0);
                match color[1] {
                    "blue" => blue += amount,
                    "red" => red += amount,
                    "green" => green += amount,
                    _ => ()
                }
            });

            game.blue.push(blue);
            game.red.push(red);
            game.green.push(green);
        });
        max_value(&game.blue)*max_value(&game.red)*max_value(&game.green)
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
