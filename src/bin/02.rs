use regex::Regex;

advent_of_code::solution!(2);

#[derive(Debug, PartialEq, Clone)]
enum Cube {
    Red,
    Green,
    Blue
}

#[derive(Debug, PartialEq, Clone)]
struct Game{
    index: u32,
    cubes: Vec<Cube>,
}

#[derive(Debug, PartialEq, Clone)]
struct Rules {
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

impl Rules {
    fn is_valid(&self, game: &Game) -> bool {
        let red_count = game.cubes.iter().filter(|cube| **cube == Cube::Red).count();
        let green_count = game.cubes.iter().filter(|cube| **cube == Cube::Green).count();
        let blue_count = game.cubes.iter().filter(|cube| **cube == Cube::Blue).count();

        red_count <= self.max_red as usize
            && green_count <= self.max_green as usize
            && blue_count <= self.max_blue as usize
    }
}

fn parse_game_data(game_data: &str, game_index_pattern: &Regex, color_count_pattern: &Regex) -> Game {
    let index = game_index_pattern.captures(game_data).unwrap()[1].parse::<u32>().unwrap();
    let red_count = color_count_pattern.captures(game_data).unwrap()[1].parse::<u32>().unwrap();
    let green_count = color_count_pattern.captures(game_data).unwrap()[1].parse::<u32>().unwrap();
    let blue_count = color_count_pattern.captures(game_data).unwrap()[1].parse::<u32>().unwrap();

    return Game {
        index,
        cubes: vec![Cube::Red; red_count as usize]
            .into_iter()
            .chain(vec![Cube::Green; green_count as usize].into_iter())
            .chain(vec![Cube::Blue; blue_count as usize].into_iter())
            .collect(),
    };
}

pub fn part_one(input: &str) -> Option<u32> {
    let rules = Rules {
        max_red: 12,
        max_green: 13,
        max_blue: 14,
    };

    let game_index_pattern = Regex::new(r"Game (\d+)").unwrap();
    let color_count_pattern = Regex::new(r"(\d+) (red|blue|green)").unwrap();

    let games = input
        .lines()
        .map(|line| parse_game_data(line, &game_index_pattern, &color_count_pattern))
        .filter(|game| rules.is_valid(game))
        .map(|game| game.index)
        .sum();

    Some(games)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
