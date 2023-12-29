use core::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0, multispace1},
    combinator::{map, opt, recognize},
    multi::many1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct Mapping {
    range_start: u64,
    range_end: u64,
    mapped_start: u64,
    mapped_end: u64,
}

#[derive(Debug, Clone)]
struct Mappings {
    mappings: Vec<Mapping>,
}

#[derive(Debug, Clone)]
struct Almanac {
    seed_to_soil: Mappings,
    soil_to_fertilizer: Mappings,
    fertilizer_to_water: Mappings,
    water_to_light: Mappings,
    light_to_temp: Mappings,
    temp_to_humidity: Mappings,
    humidity_to_location: Mappings,
}

impl Mapping {
    pub fn new(range_start: u64, mapped_start: u64, length: u64) -> Mapping {
        Mapping {
            range_start,
            range_end: range_start + length,
            mapped_start: mapped_start,
            mapped_end: mapped_start + length,
        }
    }

    pub fn get_mapped(&self, input: u64) -> Option<u64> {
        if input < self.range_start || input >= self.range_end {
            None
        } else {
            Some(self.mapped_start + (input - self.range_start))
        }
    }
}

impl Mappings {
    pub fn get_mapped(&self, input: u64) -> u64 {
        self.mappings
            .iter()
            .filter_map(|r| r.get_mapped(input))
            .next()
            .unwrap_or(input)
    }

    pub fn map_range(&self, range: Range<u64>) -> Vec<Range<u64>> {
        let mut submappings = Vec::new();

        let mut last = range.start;
        for m in &self.mappings {
            let start = m.range_start.max(range.start);
            let end = m.range_end.min(range.end);
            if end > start {
                if last < start {
                    submappings.push(last..start);
                }

                let start_diff = start - m.range_start;
                let end_diff = m.range_end - end;
                last = end + 1;
                submappings.push(m.mapped_start + start_diff..m.mapped_end - end_diff)
            }
        }

        if last < range.end {
            submappings.push(last..range.end);
        }

        submappings
    }

    pub fn map_ranges(&self, ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        ranges.into_iter().flat_map(|r| self.map_range(r)).collect()
    }
}

impl Almanac {
    pub fn new(
        seed_to_soil: Mappings,
        soil_to_fertilizer: Mappings,
        fertilizer_to_water: Mappings,
        water_to_light: Mappings,
        light_to_temp: Mappings,
        temp_to_humidity: Mappings,
        humidity_to_location: Mappings,
    ) -> Almanac {
        Almanac {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temp,
            temp_to_humidity,
            humidity_to_location,
        }
    }

    pub fn get_location(&self, seed: u64) -> u64 {
        self.humidity_to_location.get_mapped(
            self.temp_to_humidity.get_mapped(
                self.light_to_temp.get_mapped(
                    self.water_to_light.get_mapped(
                        self.fertilizer_to_water.get_mapped(
                            self.soil_to_fertilizer
                                .get_mapped(self.seed_to_soil.get_mapped(seed)),
                        ),
                    ),
                ),
            ),
        )
    }

    pub fn get_min_location(&self, range: Range<u64>) -> u64 {
        let mut ranges = vec![range];

        let mappings = vec![
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temp,
            &self.temp_to_humidity,
            &self.humidity_to_location,
        ];
        for m in mappings {
            ranges = m.map_ranges(ranges);
        }

        ranges.iter().map(|r| r.start).min().unwrap()
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (_, (seeds, almanac)) = parse(input).unwrap();

    let part_one = seeds.iter().map(|s| almanac.get_location(s.clone())).min();
    println!("Part one: {}", part_one.unwrap());
    println!("Part two: {}", calc_part_two(&seeds, &almanac).unwrap());
}

fn calc_part_two(seeds: &Vec<u64>, almanac: &Almanac) -> Option<u64> {
    (0..seeds.len())
        .step_by(2)
        .map(|i| (seeds[i], seeds[i + 1]))
        .map(|r| almanac.get_min_location(r.0..r.0 + r.1))
        .min()
}

fn parse(input: &str) -> IResult<&str, (Vec<u64>, Almanac)> {
    tuple((parse_seeds, parse_almanac))(input)
}

fn parse_int(l: &str) -> IResult<&str, u64> {
    map(recognize(digit1), |o: &str| o.parse::<u64>().unwrap())(l)
}

fn parse_seeds(l: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("seeds: "), many1(terminated(parse_int, multispace0)))(l)
}

fn parse_almanac(l: &str) -> IResult<&str, Almanac> {
    map(
        tuple((
            preceded(tuple((tag("seed-to-soil map:"), line_ending)), parse_ranges),
            preceded(
                tuple((tag("soil-to-fertilizer map:"), line_ending)),
                parse_ranges,
            ),
            preceded(
                tuple((tag("fertilizer-to-water map:"), line_ending)),
                parse_ranges,
            ),
            preceded(
                tuple((tag("water-to-light map:"), line_ending)),
                parse_ranges,
            ),
            preceded(
                tuple((tag("light-to-temperature map:"), line_ending)),
                parse_ranges,
            ),
            preceded(
                tuple((tag("temperature-to-humidity map:"), line_ending)),
                parse_ranges,
            ),
            preceded(
                tuple((tag("humidity-to-location map:"), line_ending)),
                parse_ranges,
            ),
        )),
        |p| Almanac::new(p.0, p.1, p.2, p.3, p.4, p.5, p.6),
    )(l)
}

fn parse_ranges(l: &str) -> IResult<&str, Mappings> {
    map(
        many1(terminated(parse_range, opt(line_ending))),
        |mut mappings| {
            mappings.sort_by_key(|i| i.range_start);
            Mappings { mappings }
        },
    )(l)
}

fn parse_range(l: &str) -> IResult<&str, Mapping> {
    map(
        tuple((
            terminated(parse_int, multispace1),
            terminated(parse_int, multispace1),
            terminated(parse_int, multispace0),
        )),
        |p| Mapping::new(p.1, p.0, p.2),
    )(l)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
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
56 93 4";
        let (_, (seeds, almanac)) = parse(input).unwrap();
        let part_one = seeds.into_iter().map(|s| almanac.get_location(s)).min();
        assert_eq!(35, part_one.unwrap());
    }

    #[test]
    fn test_part_two() {
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
56 93 4";
        let (_, (seeds, almanac)) = parse(input).unwrap();
        let part_two = calc_part_two(&seeds, &almanac);
        assert_eq!(46, part_two.unwrap());
    }
}
