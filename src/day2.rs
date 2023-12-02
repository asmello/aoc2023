pub mod chumsky;
pub mod manual;

// default implementation
pub use chumsky::{part1, part2};

#[derive(Debug, Default, PartialEq, Eq)]
struct Game {
    id: usize,
    draws: Vec<ColorSet>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ColorSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl ColorSet {
    pub fn new(red: usize, green: usize, blue: usize) -> Self {
        Self { red, blue, green }
    }

    fn is_empty(&self) -> bool {
        self.red == 0 && self.green == 0 && self.blue == 0
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl Game {
    fn cover(&self) -> ColorSet {
        let mut cover = ColorSet::default();
        for draw in &self.draws {
            cover.red = cover.red.max(draw.red);
            cover.green = cover.green.max(draw.green);
            cover.blue = cover.blue.max(draw.blue);
        }
        cover
    }

    fn is_possible(&self, bag: &ColorSet) -> bool {
        let cover = self.cover();
        cover.red <= bag.red && cover.green <= bag.green && cover.blue <= bag.blue
    }
}
