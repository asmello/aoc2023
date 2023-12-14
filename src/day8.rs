use crate::parse::parse;
use chumsky::prelude::*;
use miette::miette;
use std::collections::HashMap;

pub fn part1(input: &str) -> miette::Result<usize> {
    let maze = parse(input, maze())?;
    let mut curr = "AAA";
    for (count, d) in maze.instr.into_iter().cycle().enumerate() {
        if curr == "ZZZ" {
            return Ok(count);
        }

        let edges = maze
            .nodes
            .get(curr)
            .ok_or_else(|| miette!("node {curr} does not exist"))?;

        match d {
            Direction::Left => curr = edges.left,
            Direction::Right => curr = edges.right,
        }
    }
    Err(miette!("empty instructions"))
}

struct Maze<'a> {
    nodes: HashMap<&'a str, Edges<'a>>,
    instr: Vec<Direction>,
}

struct Edges<'a> {
    left: &'a str,
    right: &'a str,
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

fn maze<'a>() -> impl Parser<'a, &'a str, Maze<'a>, extra::Err<Rich<'a, char>>> {
    let nodes = node().then_ignore(text::newline()).repeated().collect();

    instructions()
        .then_ignore(text::whitespace())
        .then(nodes)
        .then_ignore(end())
        .map(|(instr, nodes)| Maze { instr, nodes })
}

fn instructions<'a>() -> impl Parser<'a, &'a str, Vec<Direction>, extra::Err<Rich<'a, char>>> {
    choice((
        just('L').to(Direction::Left),
        just('R').to(Direction::Right),
    ))
    .repeated()
    .collect()
}

fn label<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Rich<'a, char>>> {
    any()
        .filter(char::is_ascii_uppercase)
        .repeated()
        .exactly(3)
        .to_slice()
}

fn edges<'a>() -> impl Parser<'a, &'a str, Edges<'a>, extra::Err<Rich<'a, char>>> {
    label()
        .then_ignore(just(',').padded())
        .then(label())
        .map(|(left, right)| Edges { left, right })
}

fn node<'a>() -> impl Parser<'a, &'a str, (&'a str, Edges<'a>), extra::Err<Rich<'a, char>>> {
    label()
        .then_ignore(just('=').padded())
        .then(edges().delimited_by(just('('), just(')')))
}
