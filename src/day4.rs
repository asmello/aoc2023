use chumsky::{
    error::Simple,
    primitive::{choice, just},
    text::{self, TextParser},
    Parser,
};
use eyre::{eyre, Context};
use std::{
    collections::HashSet,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

pub fn part1(input: impl Read) -> eyre::Result<usize> {
    let buf_reader = BufReader::new(input);
    let mut sum = 0;
    for (num, line) in buf_reader.lines().enumerate() {
        let line = line.wrap_err_with(|| format!("could not read line {num}"))?;
        let card: Card = line
            .parse()
            .wrap_err_with(|| format!("could not parse card at line {num}"))?;
        sum += card.points();
    }
    Ok(sum)
}

#[derive(Default, Debug)]
struct Card {
    id: usize,
    winning: HashSet<usize>,
    scratched: HashSet<usize>,
}

impl Card {
    fn points(&self) -> usize {
        let matches = self.winning.intersection(&self.scratched).count();
        if matches == 0 {
            0
        } else {
            2_usize.pow((matches - 1) as u32)
        }
    }
}

impl FromStr for Card {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser()
            .parse(s)
            .map_err(|errors| eyre!(errors.into_iter().next().unwrap()))
    }
}

fn parser() -> impl Parser<char, Card, Error = Simple<char>> {
    let head = just("Card").padded();

    let integer = text::int(10).from_str::<usize>().unwrapped().padded();

    let id = integer.then_ignore(just(':')).padded();

    let number_sequence = integer.padded().repeated().collect::<HashSet<_>>();

    head.ignore_then(id)
        .then(number_sequence)
        .then_ignore(just('|'))
        .then(number_sequence)
        .map(|((id, winning), scratched)| Card {
            id,
            winning,
            scratched,
        })
}
