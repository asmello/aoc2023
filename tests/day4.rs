mod common;

use aoc2023::day4;
use indoc::indoc;

const SAMPLE: &str = indoc! { r#"
	Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
	Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
	Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
	Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
	Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
	Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 1
"# };
const INPUT_PATH: &str = "day4/input.txt";

#[test]
fn part1_sample() {
    let result = day4::part1(SAMPLE.as_bytes()).unwrap();
    assert_eq!(result, 13);
}

#[test]
fn part1_input() {
    let input = common::read(INPUT_PATH).unwrap();
    let result = day4::part1(input).unwrap();
    assert_eq!(result, 25004);
}

#[test]
fn part2_sample() {
    let result = day4::part2(SAMPLE.as_bytes()).unwrap();
    assert_eq!(result, 30);
}

#[test]
fn part2_input() {
    let input = common::read(INPUT_PATH).unwrap();
    let result = day4::part2(input).unwrap();
    assert_eq!(result, 14427616);
}
