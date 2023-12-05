use std::ops::Range;
use std::str::Lines;
advent_of_code::solution!(5);


pub struct SeedMaps(Vec<SeedMap>);
impl SeedMaps {
    pub fn map(&self, seed: u64) -> u64 {
        let mut seed = seed;
        for map in &self.0 {
            if let Some(v) = map.map(seed) {
                seed = v
            }
        }

        seed
    }
}
pub struct SeedMap(pub Vec<SeedRange>);

impl SeedMap {
    pub fn map(&self, input: u64) -> Option<u64> {
        self.0.iter()
            .filter_map(|r| r.map(input))
            .next()
    }
}

#[derive(Debug)]
pub struct SeedRange {
    pub source_range: Range<u64>,
    pub dest_range: Range<u64>
}

impl SeedRange {
    pub fn map(&self, input: u64) -> Option<u64> {
        if self.source_range.contains(&input) {
            let offset = input - self.source_range.start;
            return Some(self.dest_range.start + offset);
        }
        return None
    }
}



pub fn part_one(input: &str) -> Option<u64> {
    let (mut lines, seeds) = parse_seeds(input);
    let maps = parse_maps(&mut lines);

    let min_location = seeds.iter().map(|seed| maps.map(*seed)).min().unwrap();

    Some(min_location)
}

fn parse_seeds(input: &str) -> (Lines, Vec<u64>) {
    let mut lines = input.lines();
    let seeds = lines.next()
        .map(|seeds_line| seeds_line.split(":").last().unwrap()
            .split_ascii_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
        )
        .unwrap();
    (lines, seeds)
}

fn parse_maps(lines: &mut Lines) -> SeedMaps {
    let mut seeds_to_soils = Vec::new();
    let mut soil_to_fertilizer = Vec::new();
    let mut fertilizer_to_water = Vec::new();
    let mut water_to_light = Vec::new();
    let mut light_to_temp = Vec::new();
    let mut temp_to_humid = Vec::new();
    let mut humid_to_location = Vec::new();

    let mut current_map = &mut seeds_to_soils;

    for line in lines {
        match line {
            "seed-to-soil map:" => current_map = &mut seeds_to_soils,
            "soil-to-fertilizer map:" => current_map = &mut soil_to_fertilizer,
            "fertilizer-to-water map:" => current_map = &mut fertilizer_to_water,
            "water-to-light map:" => current_map = &mut water_to_light,
            "light-to-temperature map:" => current_map = &mut light_to_temp,
            "temperature-to-humidity map:" => current_map = &mut temp_to_humid,
            "humidity-to-location map:" => current_map = &mut humid_to_location,
            "" => {}
            numbers => {
                let mut s = numbers.split_ascii_whitespace().map(|n| n.parse::<u64>().unwrap());
                let dest_start = s.next().unwrap();
                let source_start = s.next().unwrap();
                let len = s.next().unwrap();

                let source_range = source_start..source_start + len;
                let dest_range = dest_start..dest_start + len;
                let seed_range = SeedRange {
                    source_range,
                    dest_range
                };
                current_map.push(seed_range)
            }
        }
    }

    let seeds_to_soils = SeedMap(seeds_to_soils);
    let soil_to_fertilizer = SeedMap(soil_to_fertilizer);
    let fertilizer_to_water = SeedMap(fertilizer_to_water);
    let water_to_light = SeedMap(water_to_light);
    let light_to_temp = SeedMap(light_to_temp);
    let temp_to_humid = SeedMap(temp_to_humid);
    let humid_to_location = SeedMap(humid_to_location);

    let maps = vec![
        seeds_to_soils,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humid,
        humid_to_location
    ];

    let maps = SeedMaps(maps);
    maps
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut lines, seeds) = parse_seeds(input);
    let maps = parse_maps(&mut lines);

    let seeds: Vec<Range<u64>> = seeds.chunks(2).map(|range| {
        range[0]..range[0]+range[1]
    }).collect();

    let min_location = seeds.iter().filter_map(|range| {
        range.clone().map(|seed| maps.map(seed)).min()
    })
        .min()
        .unwrap();

    Some(min_location)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
