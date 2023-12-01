advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let sum: u32 = input.lines().filter_map(|line| {
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        if digits.len() < 1 {
            None
        } else {
            let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse::<u32>()
                .unwrap();
            Some(number)
        }
    }).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // need different approach here
    // beware: there are cases like sevenine or eightwo, so regex wouldn't work here
    let patterns = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let sum = input.lines().map(|line| {
        let first = patterns.iter()
            .enumerate()
            .filter_map(|(i, pat)| {
                line.find(pat).map(|idx| (i, idx))
            })
            .min_by(|a, b| a.1.cmp(&b.1))
            .map(|(i, _)| patterns[i])
            .map(map_match)
            .unwrap();

        let last = patterns.iter()
            .enumerate()
            .filter_map(|(i, pat)| {
                line.rfind(pat).map(|idx| (i, idx))
            })
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(i, _)| patterns[i])
            .map(map_match)
            .unwrap();

        format!("{}{}", first, last).parse::<u32>().unwrap()
    })
        .sum::<u32>();
    Some(sum)
}

fn map_match(input: &str) -> &str {
    match input {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        digit => digit,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = part_one(input);

        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(360));
    }
}
