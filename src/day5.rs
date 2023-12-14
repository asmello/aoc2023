use crate::parse::parse;
use chumsky::prelude::*;
use miette::miette;
use std::collections::BTreeMap;

pub fn part1(input: &str) -> miette::Result<usize> {
    let almanac = parse(input, almanac())?;
    almanac
        .seeds
        .iter()
        .copied()
        .map(|seed| almanac.seed_to_location(seed))
        .min()
        .ok_or_else(|| miette!("empty almanac"))
}

pub fn part2(input: &str) -> miette::Result<usize> {
    let almanac = parse(input, almanac())?;
    almanac
        // TODO: optimize using range arithmetic... this is correct but, uh, slow
        .seeds()
        .map(|seed| almanac.seed_to_location(seed))
        .min()
        .ok_or_else(|| miette!("empty almanac"))
}

#[derive(Default, Debug)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil_map: BTreeMap<usize, Span>,
    soil_to_fertilizer_map: BTreeMap<usize, Span>,
    fertilizer_to_water_map: BTreeMap<usize, Span>,
    water_to_light_map: BTreeMap<usize, Span>,
    light_to_temperature_map: BTreeMap<usize, Span>,
    temperature_to_humidity_map: BTreeMap<usize, Span>,
    humidity_to_location_map: BTreeMap<usize, Span>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Span {
    start: usize,
    length: usize,
}

impl Almanac {
    fn seeds(&self) -> impl Iterator<Item = usize> + '_ {
        self.seeds.chunks_exact(2).flat_map(|pair| {
            let (initial, length) = (pair[0], pair[1]);
            initial..initial + length
        })
    }
    fn seed_to_location(&self, seed: usize) -> usize {
        let next = Self::apply(&self.seed_to_soil_map, seed);
        let next = Self::apply(&self.soil_to_fertilizer_map, next);
        let next = Self::apply(&self.fertilizer_to_water_map, next);
        let next = Self::apply(&self.water_to_light_map, next);
        let next = Self::apply(&self.light_to_temperature_map, next);
        let next = Self::apply(&self.temperature_to_humidity_map, next);
        Self::apply(&self.humidity_to_location_map, next)
    }
    fn apply(map: &BTreeMap<usize, Span>, elem: usize) -> usize {
        if let Some((&k, v)) = map.range(..=elem).last() {
            if k <= elem && k + v.length > elem {
                v.start + elem - k // elem is in span
            } else {
                elem // elem is in cover gap
            }
        } else {
            elem // elem is greater than covered maximum
        }
    }
}

fn integer<'a>() -> impl Parser<'a, &'a str, usize, extra::Err<Rich<'a, char>>> {
    text::int(10).from_str().unwrapped()
}

fn seeds<'a>() -> impl Parser<'a, &'a str, Vec<usize>, extra::Err<Rich<'a, char>>> {
    just("seeds:").ignore_then(
        integer()
            .separated_by(text::whitespace())
            .collect()
            .padded(),
    )
}

fn triple<'a>() -> impl Parser<'a, &'a str, (usize, Span), extra::Err<Rich<'a, char>>> {
    integer()
        .separated_by(text::whitespace())
        .collect_exactly::<[_; 3]>()
        .map(|[start, from, length]| (from, Span { start, length }))
}

fn map<'a>(
    label: &'static str,
) -> impl Parser<'a, &'a str, BTreeMap<usize, Span>, extra::Err<Rich<'a, char>>> {
    just(label).ignore_then(triple().separated_by(text::newline()).collect().padded())
}

fn almanac<'a>() -> impl Parser<'a, &'a str, Almanac, extra::Err<Rich<'a, char>>> {
    group((
        seeds(),
        map("seed-to-soil map:"),
        map("soil-to-fertilizer map:"),
        map("fertilizer-to-water map:"),
        map("water-to-light map:"),
        map("light-to-temperature map:"),
        map("temperature-to-humidity map:"),
        map("humidity-to-location map:"),
    ))
    .then_ignore(end())
    .map(
        |(
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        )| Almanac {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        },
    )
}
