use chumsky::{
    error::Simple,
    primitive::just,
    text::{self, TextParser},
    Parser,
};
use eyre::eyre;

pub fn part1(input: &str) -> eyre::Result<usize> {
    Ok(parse(input)?
        .into_iter()
        .map(|race| race.optimal_count())
        .product())
}

pub fn part2(input: &str) -> eyre::Result<usize> {
    todo!()
}

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn optimal_count(&self) -> usize {
        let mut count = 0;
        for hold in 1..self.time {
            let dist = hold * (self.time - hold);
            if dist > self.distance {
                count += 1;
            }
        }
        count
    }
}

fn parse(input: &str) -> eyre::Result<Vec<Race>> {
    parser()
        .parse(input)
        .map_err(|errors| eyre!(errors.into_iter().next().unwrap()))
}

fn parser() -> impl Parser<char, Vec<Race>, Error = Simple<char>> {
    let integer = text::int(10).from_str::<usize>().unwrapped().padded();

    let time = just("Time:").ignore_then(integer.repeated());

    let dist = just("Distance:").ignore_then(integer.repeated());

    time.then(dist).map(|(time, dist)| {
        time.into_iter()
            .zip(dist)
            .map(|(time, distance)| Race { time, distance })
            .collect()
    })
}
