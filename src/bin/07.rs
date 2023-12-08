use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::Map;
use std::ops::Index;
use std::str::Lines;
advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    Some(calculate_total_winnings(input, true))
}

fn parse_input(input: &str) -> Map<Lines, fn(&str) -> (&str, &str)> {
    input.lines().map(|line| {
        let mut split = line.split_ascii_whitespace();
        (split.next().unwrap(), split.next().unwrap())
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(calculate_total_winnings(input, false))
}

fn calculate_total_winnings(input: &str, is_part1: bool) -> u64 {
    let mut hands_bids = get_hands_bids_sorted(input, is_part1);
    let r = hands_bids.iter()
        .enumerate()
        .fold(0, |acc, (i, (_, (_, b)))| acc + ((i + 1) as u64 * b));
    r
}

fn get_hands_bids_sorted(input: &str, is_part1: bool) -> Vec<(usize, (String, u64))> {
    let mut hands_bids = parse_input(input)
        .map(|(hand, bid)| calculate_ranks(hand, bid, is_part1))
        .collect::<Vec<(usize, (String, u64))>>();
    hands_bids.sort_by(|(max, (hand, bid)), (bmax, (bhand, bbid))| compare_cards(max, &&**hand, bmax, bhand));
    hands_bids
}

fn calculate_ranks<'a>(hand: &'a str, bid: &'a str, is_part1: bool) -> (usize, (String, u64)) {
    let hand = if is_part1 {
        hand.replace("J", "X")
    } else {
        hand.to_string()
    };
    let mut hand = hand;
    let mut chars: Vec<char> = hand.chars().collect();
    chars.sort();
    let hands = vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 2],
        vec![1, 2, 2],
        vec![1, 1, 3],
        vec![2, 3],
        vec![1, 4],
        vec![5],
    ];

    let mut ts = Vec::new();
    for r in "J23456789TQKA".chars() {
        let r = format!("{}", r);
        let hand = hand.replace("J", r.as_str());
        let counts = hand.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        let mut values = counts.values().map(|v| *v).collect::<Vec<i32>>();
        values.sort();

        let t = hands.iter().position(|h| {
            let a = h.iter().map(|c| format!("{c}")).collect::<String>();
            let b = values.iter().map(|c| format!("{c}")).collect::<String>();
            a == b
        }).unwrap();
        ts.push(t);
    }

    let bid = bid.parse::<u64>().unwrap();
    let t = *ts.iter().max().unwrap();
    (t, (hand, bid))
}

fn compare_cards(max: &usize, hand: &&str, bmax: &usize, bhand: &String) -> Ordering {
    if max == bmax {
        let mut hand_chars = hand.chars();
        let mut bhand_chars = bhand.chars();
        (0..5).filter_map(|_| {
            let a = get_card_strength2(&hand_chars.next().unwrap());
            let b = get_card_strength2(&bhand_chars.next().unwrap());

            if a != b {
                Some(a.cmp(&b))
            } else {
                None
            }
        }).next().unwrap()
    } else {
        max.cmp(bmax)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}

fn get_card_strength(card: &char) -> i32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
     }
}
fn get_card_strength2(card: &char) -> i32 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => 0,
     }
}