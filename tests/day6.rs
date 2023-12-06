mod common;
use aoc2023::day6;
use indoc::indoc;

const SAMPLE1: &str = indoc! { r#"
	Time:      7  15   30
	Distance:  9  40  200
"# };

const SAMPLE2: &str = indoc! { r#"
    Time:      71530
    Distance:  940200
"# };

#[test]
fn part1_sample() {
    let result = day6::solve(SAMPLE1).unwrap();
    assert_eq!(result, 288);
}

#[test]
fn part1_input() {
    let input = common::read_string("day6/input.txt").unwrap();
    let result = day6::solve(&input).unwrap();
    assert_eq!(result, 512295);
}

#[test]
fn part2_sample() {
    let result = day6::solve(SAMPLE2).unwrap();
    assert_eq!(result, 71503);
}

#[test]
fn part2_input() {
    let input = common::read_string("day6/input2.txt").unwrap();
    let result = day6::solve(&input).unwrap();
    assert_eq!(result, 36530883);
}
