use chumsky::{
    error::Simple,
    primitive::{end, just},
    text::{self, newline, whitespace, TextParser},
    Parser,
};
use eyre::{eyre, Context};
use std::{collections::BTreeMap, str::FromStr};

pub fn part1(input: &str) -> eyre::Result<usize> {
    let almanac: Almanac = input.parse().wrap_err("failed to parse almanac")?;
    almanac
        .seeds
        .iter()
        .copied()
        .map(|seed| almanac.seed_to_location(seed))
        .min()
        .ok_or_else(|| eyre!("empty almanac"))
}

pub fn part2(input: &str) -> eyre::Result<usize> {
    let almanac: Almanac = input.parse().wrap_err("failed to parse almanac")?;
    // TODO: optimize using range arithmetic... this is correct but, uh, slow
    almanac
        .seeds()
        .map(|seed| almanac.seed_to_location(seed))
        .min()
        .ok_or_else(|| eyre!("empty almanac"))
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

impl FromStr for Almanac {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser()
            .parse(s)
            .map_err(|errors| eyre!(errors.into_iter().next().unwrap()))
    }
}

fn parser() -> impl Parser<char, Almanac, Error = Simple<char>> {
    let integer = text::int(10).from_str::<usize>().unwrapped();

    let seeds = just("seeds: ")
        .ignore_then(integer.separated_by(just(' ')))
        .padded();

    let triple = integer.separated_by(whitespace()).exactly(3).map(|elems| {
        (
            elems[1],
            Span {
                start: elems[0],
                length: elems[2],
            },
        )
    });

    let seed_to_soil_map = just("seed-to-soil map:")
        .padded()
        .ignore_then(triple.separated_by(newline()))
        .padded()
        .collect::<BTreeMap<_, _>>();

    let soil_to_fertilizer_map = just("soil-to-fertilizer map:")
        .padded()
        .ignore_then(triple.separated_by(newline()))
        .padded()
        .collect::<BTreeMap<_, _>>();

    let fertilizer_to_water_map = just("fertilizer-to-water map:")
        .padded()
        .ignore_then(triple.separated_by(newline()))
        .padded()
        .collect::<BTreeMap<_, _>>();

    let water_to_light_map = just("water-to-light map:")
        .padded()
        .ignore_then(triple.separated_by(newline()))
        .padded()
        .collect::<BTreeMap<_, _>>();

    let light_to_temperature_map = just("light-to-temperature map:")
        .padded()
        .ignore_then(triple.separated_by(newline()))
        .padded()
        .collect::<BTreeMap<_, _>>();

    let temperature_to_humidity_map = just("temperature-to-humidity map:")
        .padded()
        .ignore_then(triple.separated_by(newline()))
        .padded()
        .collect::<BTreeMap<_, _>>();

    let humidity_to_location_map = just("humidity-to-location map:")
        .padded()
        .ignore_then(triple.separated_by(newline()))
        .padded()
        .collect::<BTreeMap<_, _>>();

    seeds
        .then(seed_to_soil_map)
        .then(soil_to_fertilizer_map)
        .then(fertilizer_to_water_map)
        .then(water_to_light_map)
        .then(light_to_temperature_map)
        .then(temperature_to_humidity_map)
        .then(humidity_to_location_map)
        .then_ignore(end())
        .map(
            |(
                (
                    (
                        (
                            (
                                ((seeds, seed_to_soil_map), soil_to_fertilizer_map),
                                fertilizer_to_water_map,
                            ),
                            water_to_light_map,
                        ),
                        light_to_temperature_map,
                    ),
                    temperature_to_humidity_map,
                ),
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
