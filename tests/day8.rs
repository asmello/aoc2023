mod common;

use aoc2023::day8;
use indoc::indoc;

const SAMPLE1: &str = indoc! { r#"
	RL

	AAA = (BBB, CCC)
	BBB = (DDD, EEE)
	CCC = (ZZZ, GGG)
	DDD = (DDD, DDD)
	EEE = (EEE, EEE)
	GGG = (GGG, GGG)
	ZZZ = (ZZZ, ZZZ)
"# };
const SAMPLE2: &str = indoc! { r#"
	LLR

	AAA = (BBB, BBB)
	BBB = (AAA, ZZZ)
	ZZZ = (ZZZ, ZZZ)
"# };
const SAMPLE3: &str = indoc! { r#"
	LR

	11A = (11B, XXX)
	11B = (XXX, 11Z)
	11Z = (11B, XXX)
	22A = (22B, XXX)
	22B = (22C, 22C)
	22C = (22Z, 22Z)
	22Z = (22B, 22B)
	XXX = (XXX, XXX)
"# };
const INPUT_PATH: &str = "day8/input.txt";

#[test]
fn part1_samples() -> miette::Result<()> {
    let result = day8::part1(SAMPLE1)?;
    assert_eq!(result, 2);

    let result = day8::part1(SAMPLE2)?;
    assert_eq!(result, 6);

    Ok(())
}

#[test]
fn part1_input() {
    let input = common::read_string(INPUT_PATH).unwrap();
    let result = day8::part1(&input).unwrap();
    assert_eq!(result, 21389);
}

#[test]
fn part2_sample() {
    let result = day8::part2(SAMPLE3).unwrap();
    assert_eq!(result, 6);
}

#[test]
fn part2_input() {
    let input = common::read_string(INPUT_PATH).unwrap();
    let result = day8::part2(&input).unwrap();
    assert_eq!(result, 21083806112641); // wow
}
