use std::collections::HashMap;
use regex::Regex;
advent_of_code::solution!(4);


fn count_matching_numbers(r: &Regex, line: &str) -> usize {
    let numbers_str = r.captures(line).unwrap().name("numbers").unwrap().as_str();
    let mut split = numbers_str.split("|")
        .map(|mut ns| ns.split_ascii_whitespace()
            .map(|n| n.trim())
            .map(|n| n.parse::<u32>().unwrap())
        );
    let mut winning_numbers = split.next().unwrap()
        .map(|n| (n, false))
        .collect::<HashMap<u32, bool>>();
    let actual_numbers = split.next().unwrap().collect::<Vec<u32>>();

    for n in actual_numbers.iter() {
        if winning_numbers.contains_key(n) {
            let val = winning_numbers.get_mut(n).unwrap();
            *val = true;
        }
    }

    let count = winning_numbers.iter().filter(|(k, v)| **v).count();
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let r = Regex::new(r"^Card\s+\d+:\s(?P<numbers>.*)$").unwrap();
    let sum = input.lines().map(|line| {
        let count = count_matching_numbers(&r, line);
        match count {
            0 => 0,
            count => {
                2_u32.pow(count as u32-1)
            }
        }
    }).sum::<u32>();
    Some(sum)
}


pub fn part_two(input: &str) -> Option<u32> {
    let r = Regex::new(r"^Card\s+\d+:\s(?P<numbers>.*)$").unwrap();
    let match_count = input.lines().map(|line| {
        count_matching_numbers(&r, line) as u64
    }).collect::<Vec<u64>>();

    let mut num_of_cards = HashMap::with_capacity(match_count.len());
    for i in 0..match_count.len() {
        num_of_cards.insert(i, 1_u32);
    }

    match_count.iter().enumerate().for_each(|(line, count)| {
        let mut range_max = line + 1 + *count as usize;
        if range_max > match_count.len() {
            // prevent extra count of last card
            range_max = match_count.len()-1
        }
        let range = line+1 .. range_max;
        let card_count = *num_of_cards.get(&line).unwrap();
        for n in range {
            match num_of_cards.get_mut(&n) {
                None => {},
                Some(c) => {
                    let n = *c + card_count;
                    *c = n
                }
            }
        }
    });

    let sum = num_of_cards.values().sum::<u32>();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
