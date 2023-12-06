use chumsky::{
    error::Simple,
    primitive::just,
    text::{self, TextParser},
    Parser,
};
use eyre::eyre;

pub fn solve(input: &str) -> eyre::Result<usize> {
    Ok(parse(input)?
        .into_iter()
        .map(|race| race.optimal_count())
        .product())
}

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn optimal_count(&self) -> usize {
        let (t, d) = (self.time as f64, self.distance as f64);
        let delta = (t * t - 4.0 * d).sqrt();
        let h1 = ((t - delta) / 2.0).floor();
        let h2 = ((t + delta) / 2.0).ceil();
        (h2 - h1 - 1.0) as usize
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
