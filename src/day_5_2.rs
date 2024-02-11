use crate::re_utils;

use anyhow::Result;
use rayon;
use rayon::prelude::*;

struct Mapping {
    source: usize,
    destination: usize,
    range: usize,
}

pub fn process_lines(lines: Vec<String>) -> Result<usize> {
    let mut seed_to_soil_map: Vec<Mapping> = vec![];
    let mut soil_to_fertilizer: Vec<Mapping> = vec![];
    let mut fertilizer_to_water: Vec<Mapping> = vec![];
    let mut water_to_light: Vec<Mapping> = vec![];
    let mut light_to_temperature: Vec<Mapping> = vec![];
    let mut temperature_to_humidity: Vec<Mapping> = vec![];
    let mut humidity_to_location: Vec<Mapping> = vec![];

    let mut line_it = lines.iter();
    let initial_seeds = re_utils::parse_line_numbers(line_it.next().unwrap())?;
    println!("Initial seeds [{:?}]", initial_seeds);

    loop {
        let line = line_it.next();
        if line.is_none() {
            break;
        }
        let header = line.unwrap();
        if header.len() == 0 {
            continue;
        }
        println!("Parsing header [{}]", header);
        match &header[0..12] {
            "seed-to-soil" => seed_to_soil_map = parse_map(&mut line_it)?,
            "soil-to-fert" => soil_to_fertilizer = parse_map(&mut line_it)?,
            "fertilizer-t" => fertilizer_to_water = parse_map(&mut line_it)?,
            "water-to-lig" => water_to_light = parse_map(&mut line_it)?,
            "light-to-tem" => light_to_temperature = parse_map(&mut line_it)?,
            "temperature-" => temperature_to_humidity = parse_map(&mut line_it)?,
            "humidity-to-" => humidity_to_location = parse_map(&mut line_it)?,
            _ => break,
        }
    }
    let mut result = usize::MAX;
    for i in (0..initial_seeds.len()).step_by(2) {
        let (start, len) = (initial_seeds[i], initial_seeds[i + 1]);
        let loc = (start..start + len)
            .into_par_iter()
            .map(|seed| {
                let soil = find(&seed_to_soil_map, seed);
                let fertilizer = find(&soil_to_fertilizer, soil);
                let water = find(&fertilizer_to_water, fertilizer);
                let light = find(&water_to_light, water);
                let temperature = find(&light_to_temperature, light);
                let humidity = find(&temperature_to_humidity, temperature);
                let location = find(&humidity_to_location, humidity);
                location
            })
            .min();
        if let Some(location) = loc {
            if location < result {
                result = location;
            }
        }
    }

    Ok(result)
}

fn find(mapping: &Vec<Mapping>, source: usize) -> usize {
    for map in mapping {
        let upper = map.source + map.range;
        if map.source <= source && upper >= source {
            return map.destination + (source - map.source);
        }
    }
    source
}

fn parse_map(it: &mut std::slice::Iter<String>) -> Result<Vec<Mapping>> {
    let mut result: Vec<Mapping> = vec![];
    loop {
        match it.next() {
            Some(line) => {
                if line.len() == 0 {
                    break;
                }
                let (destination, source, range) = re_utils::parse_3(line)?;
                result.push(Mapping {
                    source,
                    destination,
                    range,
                });
            }
            None => break,
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    #[test]
    fn test_simple_input() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        let lines = utils::string_to_lines(input.to_string());
        let result = process_lines(lines);
        assert_eq!(46, result.unwrap());
    }
}
