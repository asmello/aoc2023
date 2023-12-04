use chumsky::{
    error::Simple,
    primitive::just,
    text::{self, TextParser},
    Parser,
};
use eyre::{eyre, Context};
use std::{
    collections::{HashMap, HashSet},
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

pub fn part2(input: impl Read) -> eyre::Result<usize> {
    let buf_reader = BufReader::new(input);
    let mut counts: HashMap<_, usize> = HashMap::new();
    for (num, line) in buf_reader.lines().enumerate() {
        let line = line.wrap_err_with(|| format!("could not read line {num}"))?;
        let card: Card = line
            .parse()
            .wrap_err_with(|| format!("could not parse card at line {num}"))?;
        // the count of copies won by past cards
        let count = counts.entry(card.id).or_default();
        *count += 1; // count the original card
        let count = *count; // drop the borrow
        let matches = card.matches();
        for id in card.id + 1..=card.id + matches {
            *counts.entry(id).or_default() += count;
        }
        // println!(
        //     "card: {} has {}, matched {} -- set is {:?}",
        //     card.id,
        //     count,
        //     card.matches(),
        //     counts
        // );
    }
    Ok(counts.into_values().sum())
}

#[derive(Default, Debug)]
struct Card {
    id: usize,
    winning: HashSet<usize>,
    scratched: HashSet<usize>,
}

impl Card {
    fn points(&self) -> usize {
        let matches = self.matches();
        if matches == 0 {
            0
        } else {
            2_usize.pow((matches - 1) as u32)
        }
    }
    fn matches(&self) -> usize {
        self.winning.intersection(&self.scratched).count()
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
