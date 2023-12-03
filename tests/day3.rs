mod common;

use aoc2023::day3;
use indoc::indoc;

const SAMPLE: &str = indoc! { r#"
	467..114..
	...*......
	..35..633.
	......#...
	617*......
	.....+.58.
	..592.....
	......755.
	...$.*....
	.664.598..
"# };

#[test]
fn part1_sample() {
    let result = day3::part1(SAMPLE.as_bytes()).unwrap();
    assert_eq!(result, 4361);
}

#[test]
fn part1_input() {
    let input = common::read("day3/input.txt").unwrap();
    let result = day3::part1(input).unwrap();
    assert_eq!(result, 540025);
}
