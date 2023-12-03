use std::{
    fmt::Display,
    io::{BufRead, BufReader, Read},
};

pub fn part1(input: impl Read) -> eyre::Result<usize> {
    let buf_reader = BufReader::new(input);
    let mut grid = Grid::default();
    let mut numbers = Vec::new();
    for (row, line) in buf_reader.lines().enumerate() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        // Extract symbols
        grid.symbols.push(
            line.chars()
                .map(|ch| !ch.is_ascii_digit() && ch != '.')
                .collect(),
        );
        // Extract numbers
        let mut maybe_start = None;
        for (col, ch) in line.chars().enumerate() {
            if let Some(start) = maybe_start {
                if !ch.is_ascii_digit() {
                    let end = col;
                    let slice = &line[start..end];
                    let value: usize = slice.parse().unwrap();
                    numbers.push(Number {
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
            let value: usize = slice.parse().unwrap();
            numbers.push(Number {
                value,
                span: Span {
                    row,
                    start,
                    end: line.len(),
                },
            });
        }
    }
    let sum = numbers
        .into_iter()
        .filter_map(|num| grid.has_adjacent_symbol(&num.span).then_some(num.value))
        .sum();
    Ok(sum)
}

#[derive(Default, Debug)]
struct Grid {
    symbols: Vec<Vec<bool>>,
}

impl Grid {
    fn width(&self) -> usize {
        if !self.symbols.is_empty() {
            self.symbols[0].len()
        } else {
            0
        }
    }
    fn height(&self) -> usize {
        self.symbols.len()
    }
    fn has_adjacent_symbol(&self, span: &Span) -> bool {
        let (start, end) = (
            span.start.saturating_sub(1),
            (span.end + 1).min(self.width()),
        );
        if self.symbols[span.row][start] || self.symbols[span.row][end - 1] {
            return true;
        }
        if span.row > 0 {
            let row = span.row - 1;
            for col in start..end {
                if self.symbols[row][col] {
                    return true;
                }
            }
        }
        if span.row + 1 < self.height() {
            let row = span.row + 1;
            for col in start..end {
                if self.symbols[row][col] {
                    return true;
                }
            }
        }
        false
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.symbols {
            for &cell in row {
                if cell {
                    f.write_str("#")?;
                } else {
                    f.write_str(".")?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
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
