use crate::parse::parse;
use chumsky::prelude::*;

pub fn solve(input: &str) -> miette::Result<usize> {
    Ok(parse(input, race())?
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

fn race<'a>() -> impl Parser<'a, &'a str, Vec<Race>, extra::Err<Rich<'a, char>>> {
    let time = just("Time:").ignore_then(integer_seq());

    let dist = just("Distance:").ignore_then(integer_seq());

    time.then(dist).map(|(time, dist)| {
        time.into_iter()
            .zip(dist)
            .map(|(time, distance)| Race { time, distance })
            .collect()
    })
}
fn integer<'a>() -> impl Parser<'a, &'a str, usize, extra::Err<Rich<'a, char>>> {
    text::int(10).from_str::<usize>().unwrapped().padded()
}

fn integer_seq<'a>() -> impl Parser<'a, &'a str, Vec<usize>, extra::Err<Rich<'a, char>>> {
    integer().repeated().collect()
}
