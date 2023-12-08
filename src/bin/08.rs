use std::collections::HashMap;
use std::str::Lines;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let (instructions, map) = parse_maps(&mut lines);

    let mut counter = 0;
    let mut start = "AAA";
    while start != "ZZZ" {
        for c in &instructions {
            let c = format!("{c}");
            let (l, r) = map.get(start).unwrap();
            if c == "L" {
                start = l.as_str();
                counter += 1;
            } else if c == "R" {
                start = r.as_str();
                counter += 1;
            }
        }
    }


    Some(counter)
}

fn parse_maps<'a>(lines: &'a mut Lines) -> (Vec<char>, HashMap<&'a str, (String, String)>) {
    let instructions = lines.next().unwrap().chars().collect::<Vec<char>>();
    let map = lines.skip(1).map(|line| {
        let mut split = line.split("=");
        let key = split.next().map(|s| s.trim()).unwrap();
        let (l, r) = split.next()
            .map(|s| s.split(","))
            .map(|mut split| (split.next().unwrap().trim(), split.next().unwrap().trim()))
            .map(|(l, r)| (l.replace("(", ""), r.replace(")", "")))
            .unwrap();
        (key, (l, r))
    }).collect::<HashMap<&str, (String, String)>>();
    (instructions, map)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let (instructions, map) = parse_maps(&mut lines);

    let mut starts = map.keys()
        .filter(|k| k.get(2..3) == Some("A"))
        .map(|k| *k).collect::<Vec<&str>>();

    let kgv = starts.iter()
        .map(|start| track_start(start, &instructions, &map))
        .map(|(s, c)| c as usize)
        .map(|c| c / instructions.len())
        .fold(1, |acc, c| acc * c);

    let result = (kgv * instructions.len()) as u64;

    Some(result)
}
fn track_start<'a>(start: &'a str, instructions: &Vec<char>, map: &'a HashMap<&'a str, (String, String)>) -> (&'a str, u32) {
    let mut start = start;
    let mut counter = 0;
    while start.get(2..3) != Some("Z") {
        for c in instructions {
            let c = format!("{c}");
            let (l, r) = map.get(start).unwrap();
            if c == "L" {
                start = l.as_str();
                counter += 1;
            } else if c == "R" {
                start = r.as_str();
                counter += 1;
            }
        }
    }
    (start, counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = part_two(input);
        assert_eq!(result, Some(6));
    }
}
