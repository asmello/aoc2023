use super::{ColorSet, Game};
use crate::parse::parse;
use chumsky::{prelude::*, text};
use miette::Context;

pub fn part1(input: &str, bag: &ColorSet) -> miette::Result<usize> {
    let mut sum = 0;
    for (num, line) in input.lines().enumerate() {
        let game = parse(line, parser()).wrap_err_with(|| format!("at line {num}"))?;
        if game.is_possible(bag) {
            sum += game.id;
        }
    }
    Ok(sum)
}

pub fn part2(input: &str) -> miette::Result<usize> {
    let mut sum = 0;
    for (num, line) in input.lines().enumerate() {
        let game = parse(line, parser()).wrap_err_with(|| format!("at line {num}"))?;
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

fn parser<'a>() -> impl Parser<'a, &'a str, Game, extra::Err<Rich<'a, char>>> {
    let head = just("Game").padded();

    let integer = text::int(10).from_str().unwrapped().padded();

    let id = integer.then_ignore(just(':')).padded();

    let color = choice((
        just("red").to(Color::Red),
        just("green").to(Color::Green),
        just("blue").to(Color::Blue),
    ));

    let color_count = integer.then(color);

    let color_set = color_count
        .separated_by(just(','))
        .collect()
        .map(|color_counts: Vec<_>| {
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

    let color_sets = color_set.separated_by(just(';')).collect();

    head.ignore_then(id)
        .then(color_sets)
        .then_ignore(end())
        .map(|(id, draws)| Game { id, draws })
}

#[cfg(test)]
mod tests {
    use super::{parser, ColorSet, Game};
    use crate::parse::parse;

    #[test]
    fn can_parse() {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse(s, parser()).unwrap();
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
