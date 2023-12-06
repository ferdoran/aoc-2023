advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let times = lines.next()
        .map(|line| line.split(":").last().unwrap().split_ascii_whitespace().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>())
        .unwrap();
    let distances = lines.next()
        .map(|line| line.split(":").last().unwrap().split_ascii_whitespace().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>())
        .unwrap();

    println!("times: {:?}", times);
    println!("distances: {:?}", distances);


    let result = times.iter().enumerate().map(|(i, time)| {
        let record = distances[i];

        let numbers_of_ways = (0..*time).filter_map(|t| {
            let d = (*time - t) * t;
            if d > record {
                Some(d)
            } else {
                None
            }
        })
            .collect::<Vec<u64>>();
        numbers_of_ways.len()
    })
            .fold(1, |acc, x| acc * x);

    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let times = lines.next()
        .map(|line| line.split(":").last().unwrap().split_ascii_whitespace().collect::<String>())
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap();
    let distances = lines.next()
        .map(|line| line.split(":").last().unwrap().split_ascii_whitespace().collect::<String>())
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap();

    println!("times: {:?}", times);
    println!("distances: {:?}", distances);

    let record = distances;

    let numbers_of_ways = (0..times).filter_map(|t| {
        let d = (times - t) * t;
        if d > record {
            Some(d)
        } else {
            None
        }
    })
        .collect::<Vec<u64>>();
    let result = numbers_of_ways.len();


    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
