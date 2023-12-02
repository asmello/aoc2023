mod common;

use aoc2023::day2::{self, ColorSet};
use indoc::indoc;

#[test]
fn part1_sample() {
    const INPUT: &str = indoc! { r#"
		Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
		Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
		Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
		Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
		Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
	"# };
    let bag = ColorSet::new(12, 13, 14);
    let result = day2::part1(INPUT.as_bytes(), &bag).unwrap();
    assert_eq!(result, 8);
}

#[test]
fn part1_input() {
    let input = common::read("day2/input.txt").unwrap();
    let bag = ColorSet::new(12, 13, 14);
    let result = day2::part1(input, &bag).unwrap();
    assert_eq!(result, 2076);
}
