mod common;

use indoc::indoc;

const SAMPLE: &str = indoc! { r#"
		Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
		Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
		Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
		Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
		Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
	"# };

mod manual {
    use super::SAMPLE;
    use crate::common;
    use aoc2023::day2::{
        manual::{part1, part2},
        ColorSet,
    };

    #[test]
    fn part1_sample() {
        let bag = ColorSet::new(12, 13, 14);
        let result = part1(SAMPLE, &bag).unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn part1_input() {
        let input = common::read_string("day2/input.txt").unwrap();
        let bag = ColorSet::new(12, 13, 14);
        let result = part1(&input, &bag).unwrap();
        assert_eq!(result, 2076);
    }

    #[test]
    fn part2_sample() {
        let result = part2(SAMPLE).unwrap();
        assert_eq!(result, 2286);
    }

    #[test]
    fn part2_input() {
        let input = common::read_string("day2/input.txt").unwrap();
        let result = part2(&input).unwrap();
        assert_eq!(result, 70950);
    }
}

mod chumsky {
    use super::SAMPLE;
    use crate::common;
    use aoc2023::day2::{
        chumsky::{part1, part2},
        ColorSet,
    };

    #[test]
    fn part1_sample() {
        let bag = ColorSet::new(12, 13, 14);
        let result = part1(SAMPLE, &bag).unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn part1_input() {
        let input = common::read_string("day2/input.txt").unwrap();
        let bag = ColorSet::new(12, 13, 14);
        let result = part1(&input, &bag).unwrap();
        assert_eq!(result, 2076);
    }

    #[test]
    fn part2_sample() {
        let result = part2(SAMPLE).unwrap();
        assert_eq!(result, 2286);
    }

    #[test]
    fn part2_input() {
        let input = common::read_string("day2/input.txt").unwrap();
        let result = part2(&input).unwrap();
        assert_eq!(result, 70950);
    }
}
