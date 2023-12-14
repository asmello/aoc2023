use crate::parse::parse;
use chumsky::prelude::*;
use miette::miette;
use std::collections::HashMap;

pub fn part1(input: &str) -> miette::Result<usize> {
    let maze = parse(input, maze())?;
    steps(&maze, "AAA")
}

pub fn part2(input: &str) -> miette::Result<usize> {
    let maze = parse(input, maze())?;
    let steps = maze
        .starts()
        .map(|node| steps(&maze, node))
        .collect::<miette::Result<Vec<_>>>()?;
    steps
        .into_iter()
        .reduce(lcm)
        .ok_or_else(|| miette!("empty set"))
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn steps(maze: &Maze, start: &str) -> miette::Result<usize> {
    let mut curr = start;
    for (count, dir) in maze.instructions().enumerate() {
        if curr.ends_with('Z') {
            return Ok(count);
        }
        curr = maze.apply(curr, dir)?;
    }
    Err(miette!("empty instructions"))
}

struct Maze<'a> {
    nodes: HashMap<&'a str, Edges<'a>>,
    instr: Vec<Direction>,
}

impl<'a> Maze<'a> {
    fn starts(&self) -> impl Iterator<Item = &'a str> + '_ {
        self.nodes.keys().filter(|key| key.ends_with('A')).copied()
    }
    fn instructions(&self) -> impl Iterator<Item = Direction> + '_ {
        self.instr.iter().cycle().copied()
    }
    fn apply(&self, node: &'a str, dir: Direction) -> miette::Result<&'a str> {
        let edges = self
            .nodes
            .get(node)
            .ok_or_else(|| miette!("node {node} does not exist"))?;

        Ok(match dir {
            Direction::Left => edges.left,
            Direction::Right => edges.right,
        })
    }
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
        .filter(char::is_ascii_alphanumeric)
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
