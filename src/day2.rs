use eyre::{bail, ensure, eyre, Context};
use std::{
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<ColorSet>,
}

#[derive(Debug, Default)]
pub struct ColorSet {
    red: usize,
    green: usize,
    blue: usize,
}

pub fn part1(input: impl Read, bag: &ColorSet) -> eyre::Result<usize> {
    let buf_reader = BufReader::new(input);
    let mut sum = 0;
    for (num, line) in buf_reader.lines().enumerate() {
        let line = line?;
        let game: Game = line
            .parse()
            .wrap_err_with(|| format!("failed to parse game at line {num}"))?;
        if game.is_possible(bag) {
            sum += game.id;
        }
    }
    Ok(sum)
}

impl ColorSet {
    pub fn new(red: usize, green: usize, blue: usize) -> Self {
        Self { red, blue, green }
    }

    fn is_empty(&self) -> bool {
        self.red == 0 && self.green == 0 && self.blue == 0
    }
}

impl Game {
    fn is_possible(&self, bag: &ColorSet) -> bool {
        let mut max = ColorSet::default();
        for draw in &self.draws {
            max.red = max.red.max(draw.red);
            max.green = max.green.max(draw.green);
            max.blue = max.blue.max(draw.blue);
        }
        max.red <= bag.red && max.green <= bag.green && max.blue <= bag.blue
    }
}

impl FromStr for Game {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();
        let game = tokens.next().ok_or_else(|| eyre!("expected Game token"))?;
        ensure!(game == "Game", "invalid token {game}, expected Game");
        let id = tokens
            .next()
            .ok_or_else(|| eyre!("expected game id token"))?;
        let id = parse_id(id)?;
        let draws = parse_draws(&mut tokens)?;
        Ok(Self { id, draws })
    }
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
