use super::{ColorSet, Game};
use chumsky::{prelude::*, text};
use eyre::{eyre, Context};
use std::io::{BufRead, BufReader, Read};

pub fn part1(input: impl Read, bag: &ColorSet) -> eyre::Result<usize> {
    let buf_reader = BufReader::new(input);
    let mut sum = 0;
    for (num, line) in buf_reader.lines().enumerate() {
        let line = line?;
        let game =
            parse_game(&line).wrap_err_with(|| format!("failed to parse game at line {num}"))?;
        if game.is_possible(bag) {
            sum += game.id;
        }
    }
    Ok(sum)
}

pub fn part2(input: impl Read) -> eyre::Result<usize> {
    let buf_reader = BufReader::new(input);
    let mut sum = 0;
    for (num, line) in buf_reader.lines().enumerate() {
        let line = line?;
        let game =
            parse_game(&line).wrap_err_with(|| format!("failed to parse game at line {num}"))?;
        let cover = game.cover();
        sum += cover.power();
    }
    Ok(sum)
}

#[derive(Clone, Copy, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

fn parse_game(s: &str) -> eyre::Result<Game> {
    parser()
        .parse(s)
        .map_err(|errors| eyre!(errors.into_iter().next().unwrap()))
}

fn parser() -> impl Parser<char, Game, Error = Simple<char>> {
    let head = just("Game").padded();

    let id = text::int(10)
        .map(|s: String| s.parse::<usize>().unwrap())
        .then_ignore(just(':'));

    let color = just("red")
        .to(Color::Red)
        .or(just("green").to(Color::Green))
        .or(just("blue").to(Color::Blue));

    let color_count = text::int(10)
        .map(|s: String| s.parse::<usize>().unwrap())
        .padded()
        .then(color);

    let color_set = color_count.separated_by(just(',')).map(|color_counts| {
        let mut set = ColorSet::default();
        for (count, color) in color_counts {
            match color {
                Color::Red => set.red += count,
                Color::Green => set.green += count,
                Color::Blue => set.blue += count,
            }
        }
        set
    });

    let color_set = color_set.separated_by(just(';'));

    head.ignore_then(id)
        .then(color_set)
        .map(|(id, draws)| Game { id, draws })
}

#[cfg(test)]
mod tests {
    use crate::day2::{ColorSet, Game};

    use super::parse_game;

    #[test]
    fn parse() {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse_game(s).unwrap();
        let expected = Game {
            id: 1,
            draws: vec![
                ColorSet {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                ColorSet {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                ColorSet {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        };
        assert_eq!(game, expected);
    }
}
