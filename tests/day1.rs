mod common;

use aoc2023::day1;
use indoc::indoc;

#[test]
fn part1_sample() {
    const SAMPLE: &str = indoc! { r#"
		1abc2
		pqr3stu8vwx
		a1b2c3d4e5f
		treb7uchet
	"# };

    let result = day1::part1(SAMPLE.as_bytes()).unwrap();
    assert_eq!(result, 142);
}

#[test]
fn part1_input() {
    let input = common::read("day1/input.txt").unwrap();
    let result = day1::part1(input).unwrap();
    assert_eq!(result, 54632);
}

#[test]
fn part2_sample() {
    const SAMPLE: &str = indoc! { r#"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
	"# };

    let result = day1::part2(SAMPLE.as_bytes()).unwrap();
    assert_eq!(result, 281);
}

#[test]
fn part2_special() {
    // this is dumb
    let input = "sevenine";
    let result = day1::part2(input.as_bytes()).unwrap();
    assert_eq!(result, 79);
}

#[test]
fn part2_input() {
    let input = common::read("day1/input.txt").unwrap();
    let result = day1::part2(input).unwrap();
    assert_eq!(result, 54019);
}
