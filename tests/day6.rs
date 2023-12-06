mod common;
use aoc2023::day6;
use indoc::indoc;

const SAMPLE: &str = indoc! { r#"
	Time:      7  15   30
	Distance:  9  40  200
"# };
const INPUT_PATH: &str = "day6/input.txt";

#[test]
fn part1_sample() {
    let result = day6::part1(SAMPLE).unwrap();
    assert_eq!(result, 288);
}

#[test]
fn part1_input() {
    let input = common::read_string(INPUT_PATH).unwrap();
    let result = day6::part1(&input).unwrap();
    assert_eq!(result, 0);
}

// #[test]
// fn part2_sample() {
//     let result = day6::part2(SAMPLE).unwrap();
//     assert_eq!(result, 0);
// }

// #[test]
// fn part2_input() {
//     let input = common::read_string(INPUT_PATH).unwrap();
//     let result = day6::part2(&input).unwrap();
//     assert_eq!(result, 0);
// }
