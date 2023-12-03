advent_of_code::solution!(3);


const GRID_SIZE: usize = 140;

pub fn part_one(input: &str) -> Option<u32> {
    let mut schematic = Vec::new();
    let mut symbols = Vec::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            schematic.push(c);
            // schematic[i][j] = c;
            if c != '.' && c.is_ascii_punctuation() {
                symbols.push((i, j));
                // println!("symbol location: {}, {}", i, j);
            }
        });
    });

    let mut digit_buffer = Vec::new();
    let mut numbers = Vec::new();
    for y in 0..GRID_SIZE {
        if !digit_buffer.is_empty() {
            let (num_len, number) = match number_from_digit_buffer(&mut digit_buffer) {
                Some(value) => value,
                None => continue,
            };
            check_adjacent_symbols(&mut schematic, &mut numbers, y - 1, GRID_SIZE - 1, num_len - 1, number);
            digit_buffer.clear();
        }
        for x in 0..GRID_SIZE {
            let c = schematic[y * GRID_SIZE + x];
            if c.is_ascii_digit() {
                digit_buffer.push(c)
            } else if c.is_ascii_punctuation() {
                // number ends
                let (num_len, number) = match number_from_digit_buffer(&mut digit_buffer) {
                    Some(value) => value,
                    None => continue,
                };
                if c == '.' {
                    check_adjacent_symbols(&mut schematic, &mut numbers, y, x, num_len, number);
                    digit_buffer.clear();
                } else {
                    numbers.push(number);
                    digit_buffer.clear();
                }
            }
        }
    }
    let sum = numbers.iter().sum::<u32>();
    Some(sum)
}

fn check_adjacent_symbols(schematic: &mut Vec<char>, numbers: &mut Vec<u32>, y: usize, x: usize, num_len: usize, number: u32) {
    let mut start_x = 0;
    if (x as i32 - num_len as i32 - 1) < 0 {
        start_x = 0;
    } else {
        start_x = x - num_len - 1;
    }
    let mut has_upper_symbol = false;
    if y > 0 {
        let idx = (y - 1) * GRID_SIZE + start_x..=(y - 1) * GRID_SIZE + x;
        has_upper_symbol = schematic[idx].iter().any(|ch| *ch != '.' && ch.is_ascii_punctuation());
    }

    let mut has_lower_symbol = false;
    if y < GRID_SIZE - 1 {
        let idx = (y + 1) * GRID_SIZE + start_x..=(y + 1) * GRID_SIZE + x;
        has_lower_symbol = schematic[idx].iter().any(|ch| *ch != '.' && ch.is_ascii_punctuation());
    }

    let lefter_idx = y * GRID_SIZE + start_x..=y * GRID_SIZE + x;
    let has_lefter_symbol = schematic[lefter_idx].iter().any(|ch| *ch != '.' && ch.is_ascii_punctuation());


    if has_upper_symbol || has_lower_symbol || has_lefter_symbol {
        numbers.push(number);
    }
}

fn number_from_digit_buffer(digit_buffer: &mut Vec<char>) -> Option<(usize, u32)> {
    let num_len = digit_buffer.len();
    if num_len == 0 {
        return None;
    }
    let number = digit_buffer.iter().collect::<String>().parse::<u32>().unwrap();
    Some((num_len, number))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut schematic = Vec::new();
    let mut symbols = Vec::new();
    let mut numbers = Vec::new();
    input.lines().enumerate().for_each(|(i, line)| {
        let mut digit_buffer = Vec::new();
        line.chars().enumerate().for_each(|(j, c)| {
            schematic.push(c);
            // schematic[i][j] = c;
            if c.is_ascii_punctuation() {
                if c != '.' {
                    symbols.push((c, (i, j)));
                }
                if digit_buffer.len() > 0 {
                    let len = digit_buffer.len();
                    let number = digit_buffer.iter().collect::<String>().parse::<u32>().unwrap();
                    let min_j = if j as i32 - len as i32 - 1 < 0 {
                        0
                    } else {
                        j - len - 1
                    };
                    numbers.push((number, (i, min_j..j - 1)));
                    digit_buffer.clear();
                }
                // println!("symbol location: {}, {}", i, j);
            } else if c.is_ascii_digit() {
                digit_buffer.push(c);
            }

            if j == GRID_SIZE - 1 && !digit_buffer.is_empty() {
                let len = digit_buffer.len();
                let number = digit_buffer.iter().collect::<String>().parse::<u32>().unwrap();
                numbers.push((number, (i, j - len..j)));
                digit_buffer.clear();
            }
        });
    });

    let sum = symbols.iter()
        .filter(|(c, (y, x))| *c == '*')
        .filter_map(|(_, (y, x))| {
            let y_range = y - 1..=y + 1;
            let x_range = x - 1..x + 1;
            let nums: Vec<u32> = numbers.iter().filter_map(|(num, (ny, nx))| {
                if y_range.contains(ny) && (x_range == *nx || (x_range.start <= nx.end && nx.start < x_range.end)) {
                    Some(*num)
                } else {
                    None
                }
            }).collect();
            if nums.len() == 2 {
                Some(nums[0] * nums[1])
            } else {
                None
            }
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
