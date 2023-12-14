mod common;
use aoc2023::day5;
use indoc::indoc;

const SAMPLE: &str = indoc! { r#"
    seeds: 79 14 55 13

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
"# };
const INPUT_PATH: &str = "day5/input.txt";

#[test]
fn part1_sample() {
    let result = day5::part1(SAMPLE).unwrap();
    assert_eq!(result, 35);
}

#[test]
fn part1_input() {
    let input = common::read_string(INPUT_PATH).unwrap();
    let result = day5::part1(&input).unwrap();
    assert_eq!(result, 535088217);
}

#[test]
fn part2_sample() {
    let result = day5::part2(SAMPLE).unwrap();
    assert_eq!(result, 46);
}

// #[test]
// fn part2_input() {
//     let input = common::read_string(INPUT_PATH).unwrap();
//     let result = day5::part2(&input).unwrap();
//     assert_eq!(result, 51399228);
// }
