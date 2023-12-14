use chumsky::prelude::*;
use miette::Context;
use std::collections::{HashMap, HashSet};

use crate::parse::parse;

pub fn part1(input: &str) -> miette::Result<usize> {
    let mut sum = 0;
    for (num, line) in input.lines().enumerate() {
        let card = parse(line, parser()).wrap_err_with(|| format!("at line {num}"))?;
        sum += card.points();
    }
    Ok(sum)
}

pub fn part2(input: &str) -> miette::Result<usize> {
    let mut counts: HashMap<_, usize> = HashMap::new();
    for (num, line) in input.lines().enumerate() {
        let card = parse(line, parser()).wrap_err_with(|| format!("at line {num}"))?;
        // the count of copies won by past cards
        let count = counts.entry(card.id).or_default();
        *count += 1; // count the original card
        let count = *count; // drop the borrow
        let matches = card.matches();
        for id in card.id + 1..=card.id + matches {
            *counts.entry(id).or_default() += count;
        }
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

fn parser<'a>() -> impl Parser<'a, &'a str, Card, extra::Err<Rich<'a, char>>> {
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
