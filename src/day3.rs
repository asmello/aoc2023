use std::io::{BufRead, BufReader, Read};

use eyre::Context;

pub fn part1(input: impl Read) -> eyre::Result<usize> {
    let grid = build_grid(input)?;
    let sum = grid
        .numbers
        .iter()
        .filter_map(|num| grid.has_adjacent_symbol(&num.span).then_some(num.value))
        .sum();
    Ok(sum)
}

pub fn part2(input: impl Read) -> eyre::Result<usize> {
    let grid = build_grid(input)?;
    let sum = grid
        .iter()
        .filter_map(|(row, col, _)| grid.gear_ratio(row, col))
        .sum();
    Ok(sum)
}

fn build_grid(input: impl Read) -> eyre::Result<Grid> {
    let buf_reader = BufReader::new(input);
    let mut grid = Grid::default();
    for (row, line) in buf_reader.lines().enumerate() {
        let line = line.wrap_err_with(|| format!("failed to read line {row}"))?;
        if line.is_empty() {
            continue;
        }
        let mut cells = Vec::with_capacity(line.len());
        let mut maybe_start = None;
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                // Number cells have the index where the number will be in the numbers vector
                cells.push(GridCell::Number(grid.numbers.len()));
            } else if ch != '.' {
                cells.push(GridCell::Symbol(ch));
            } else {
                cells.push(GridCell::Empty);
            }
            if let Some(start) = maybe_start {
                if !ch.is_ascii_digit() {
                    let end = col;
                    let slice = &line[start..end];
                    let value: usize = slice
                        .parse()
                        .wrap_err("could not parse sequence as usize at line {row} col {col}")?;
                    grid.numbers.push(Number {
                        value,
                        span: Span { row, start, end },
                    });
                    maybe_start = None;
                }
            } else if ch.is_ascii_digit() {
                maybe_start = Some(col);
            }
        }
        if let Some(start) = maybe_start {
            let slice = &line[start..];
            let value: usize = slice
                .parse()
                .wrap_err("could not parse sequence as usize at line {row} col {col}")?;
            grid.numbers.push(Number {
                value,
                span: Span {
                    row,
                    start,
                    end: line.len(),
                },
            });
        }
        grid.cells.push(cells);
    }
    Ok(grid)
}

#[derive(Debug, Clone)]
enum GridCell {
    Number(usize),
    Symbol(char),
    Empty,
}

impl GridCell {
    fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(_))
    }
}

#[derive(Debug)]
struct Number {
    value: usize,
    span: Span,
}

#[derive(Debug)]
struct Span {
    row: usize,
    start: usize,
    end: usize,
}

#[derive(Default, Debug)]
struct Grid {
    cells: Vec<Vec<GridCell>>,
    numbers: Vec<Number>,
}

impl Grid {
    fn width(&self) -> usize {
        if !self.cells.is_empty() {
            self.cells[0].len()
        } else {
            0
        }
    }
    fn height(&self) -> usize {
        self.cells.len()
    }
    fn has_adjacent_symbol(&self, span: &Span) -> bool {
        let (start, end) = (
            span.start.saturating_sub(1),
            (span.end + 1).min(self.width()),
        );
        let check_row = |row: usize| {
            for col in start..end {
                if self.cells[row][col].is_symbol() {
                    return true;
                }
            }
            false
        };
        if self.cells[span.row][start].is_symbol() || self.cells[span.row][end - 1].is_symbol() {
            return true;
        }
        if span.row > 0 && check_row(span.row - 1) {
            return true;
        }
        if span.row + 1 < self.height() && check_row(span.row + 1) {
            return true;
        }
        false
    }
    fn neighbours(&self, row: usize, col: usize) -> impl Iterator<Item = &GridCell> {
        const DELTAS: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        DELTAS.iter().filter_map(move |(d_row, d_col)| {
            let (new_row, new_col) = (row as i32 + d_row, col as i32 + d_col);
            (new_row >= 0
                && new_col >= 0
                && (new_row as usize) < self.height()
                && (new_col as usize) < self.width())
            .then_some(&self.cells[new_row as usize][new_col as usize])
        })
    }
    fn gear_ratio(&self, row: usize, col: usize) -> Option<usize> {
        if !matches!(self.cells[row][col], GridCell::Symbol('*')) {
            return None;
        }
        let mut neighbour_numbers: Vec<_> = self
            .neighbours(row, col)
            .filter_map(|cell| match cell {
                GridCell::Number(index) => Some(*index),
                _ => None,
            })
            .collect();
        neighbour_numbers.dedup(); // Numbers may occupy multiple cells
        (neighbour_numbers.len() == 2).then(|| {
            neighbour_numbers
                .into_iter()
                .map(|idx| self.numbers[idx].value)
                .product()
        })
    }
    fn iter(&self) -> impl Iterator<Item = (usize, usize, &GridCell)> {
        self.cells.iter().enumerate().flat_map(|(row, row_cells)| {
            row_cells
                .iter()
                .enumerate()
                .map(move |(col, cell)| (row, col, cell))
        })
    }
}
