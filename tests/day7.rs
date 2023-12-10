mod common;
use aoc2023::day7;
use indoc::indoc;

const SAMPLE: &str = indoc! { r#"
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
"# };
const INPUT_PATH: &str = "day7/input.txt";

#[test]
fn part1_sample() {
    let result = day7::part1(SAMPLE).unwrap();
    assert_eq!(result, 6440);
}

#[test]
fn part1_input() {
    let input = common::read_string(INPUT_PATH).unwrap();
    let result = day7::part1(&input).unwrap();
    assert_eq!(result, 250058342);
}

#[test]
fn part2_sample() {
    let result = day7::part2(SAMPLE).unwrap();
    assert_eq!(result, 5905);
}

#[test]
fn part2_input() {
    let input = common::read_string(INPUT_PATH).unwrap();
    let result = day7::part2(&input).unwrap();
    assert_eq!(result, 250506580);
}
