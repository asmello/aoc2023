use eyre::{bail, ensure, eyre, Context};
use std::{
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

use super::{ColorSet, Game};

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

fn parse_game(s: &str) -> eyre::Result<Game> {
    let mut tokens = s.split_ascii_whitespace();
    let game = tokens.next().ok_or_else(|| eyre!("expected Game token"))?;
    ensure!(game == "Game", "invalid token {game}, expected Game");
    let id = tokens
        .next()
        .ok_or_else(|| eyre!("expected game id token"))?;
    let id = parse_id(id)?;
    let draws = parse_draws(&mut tokens)?;
    Ok(Game { id, draws })
}

fn parse_id(s: &str) -> eyre::Result<usize> {
    for (pos, ch) in s.chars().enumerate() {
        if ch.is_ascii_digit() {
            continue;
        }
        if ch == ':' {
            return s[..pos].parse().wrap_err("failed to parse as usize");
        }
        return Err(eyre!("invalid character {ch} at position {pos}"));
    }
    Err(eyre!("missing terminator :"))
}

fn parse_draws<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> eyre::Result<Vec<ColorSet>> {
    let mut draws = Vec::new();
    while let Some(draw) = parse_draw(tokens)? {
        draws.push(draw);
    }
    Ok(draws)
}

fn parse_draw<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> eyre::Result<Option<ColorSet>> {
    let mut set = ColorSet::default();
    while let Some(token) = parse_color(tokens)? {
        match token.color {
            Color::Red => set.red += token.count,
            Color::Green => set.green += token.count,
            Color::Blue => set.blue += token.count,
        }
        if token.is_terminal {
            break;
        }
    }
    if set.is_empty() {
        Ok(None)
    } else {
        Ok(Some(set))
    }
}

struct ColorRecord {
    count: usize,
    color: Color,
    is_terminal: bool,
}

enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            other => bail!("invalid color: {other}"),
        })
    }
}

fn parse_color<'a>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> eyre::Result<Option<ColorRecord>> {
    let Some(count) = tokens.next() else {
        return Ok(None);
    };
    let count = count.parse::<usize>()?;
    let color = tokens.next().ok_or_else(|| eyre!("expecting color name"))?;
    let (color, is_terminal) = match color.chars().last() {
        Some(',') => (&color[..color.len() - 1], false),
        Some(';') => (&color[..color.len() - 1], true),
        Some(_) => (color, true),
        None => bail!("empty color name"),
    };
    let color = color.parse()?;
    Ok(Some(ColorRecord {
        count,
        color,
        is_terminal,
    }))
}

#[cfg(test)]
mod tests {
    use super::parse_game;
    use indoc::indoc;

    #[test]
    fn power_of_cover() {
        const INPUT: &str = indoc! { r#"
    		Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    		Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    		Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    		Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    		Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    	"# };

        let mut powers = Vec::new();
        for line in INPUT.lines() {
            let game = parse_game(line).unwrap();
            let cover = game.cover();
            powers.push(cover.power());
        }
        assert_eq!(powers, [48, 12, 1560, 630, 36]);
    }
}
