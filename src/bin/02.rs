use std::iter::FilterMap;
use std::str::Lines;
use regex::{Captures, Regex};
use crate::CubeColor::{Blue, Green, Red};
advent_of_code::solution!(2);

pub enum CubeColor {
    Red(u32),
    Green(u32),
    Blue(u32)
}

#[derive(Default)]
pub struct CubeConfig {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl CubeConfig {
    pub fn is_valid(&self, other: &CubeConfig) -> bool {
        other.red <= self.red && other.green <= self.green && other.blue <= self.blue
    }

    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl From<(&str, &str)> for CubeColor {
    fn from(value: (&str, &str)) -> Self {
        let (num, color) = value;
        let num = num.parse::<u32>().expect("failed to parse cube num");
        match color {
            "red" => Red(num),
            "green" => Green(num),
            "blue" => Blue(num),
            _ => panic!("invalid color: {}", color)
        }
    }
}


fn captures_to_cube_configs(captures: Captures) -> (u32, Vec<CubeConfig>) {
    let game_id = captures.name("game")
        .expect("failed to get game").as_str()
        .parse::<u32>()
        .expect("failed to parse game_id");
    let cubes = captures.name("cubes").expect("failed to get cubes").as_str();

    let configs = cubes.split(";")
        .map(|subset| {
            let mut cube_config = CubeConfig::default();
            subset.split(",")
                .map(|cube| {
                    let mut split = cube.trim().split_ascii_whitespace();
                    let cube = (split.next().unwrap(), split.next().unwrap());
                    CubeColor::from(cube)
                })
                .for_each(|cube_color| {
                    match cube_color {
                        Red(n) => { cube_config.red += n }
                        Green(n) => { cube_config.green += n }
                        Blue(n) => { cube_config.blue += n }
                    };
                });
            cube_config
        }).collect::<Vec<CubeConfig>>();
    (game_id, configs)
}

fn calculate_min_config(configs: &Vec<CubeConfig>) -> CubeConfig {
    let red = configs.iter().map(|c| c.red).max().unwrap();
    let green = configs.iter().map(|c| c.green).max().unwrap();
    let blue = configs.iter().map(|c| c.blue).max().unwrap();

    CubeConfig {
        red,
        green,
        blue
    }
}

fn input_to_captures(input: &str) -> FilterMap<Lines, fn(&str) -> Option<Captures>> {
    let r = Regex::new(r"^Game\s(?P<game>\d{1,3}):\s(?P<cubes>.*)$").unwrap();
    input.lines()
        .filter_map(|line| {
            // println!("line: {}", line);
            r.captures(line)
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    let config = CubeConfig {
        red: 12,
        green: 13,
        blue: 14
    };
    let sum = input_to_captures(input)
        .filter_map(|captures| {
            let (game_id, configs) = captures_to_cube_configs(captures);

            let is_valid = configs.iter()
                .all(|cube_config| config.is_valid(&cube_config));

            if is_valid {
                Some(game_id)
            } else {
                None
            }
        })
        .sum::<u32>();
    Some(sum)
}


pub fn part_two(input: &str) -> Option<u32> {
    let sum = input_to_captures(input)
        .map(|captures| {
            let (_, configs) = captures_to_cube_configs(captures);
            calculate_min_config(&configs).power()
        })
        .sum::<u32>();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
