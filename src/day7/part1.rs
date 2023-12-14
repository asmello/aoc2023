use miette::{miette, IntoDiagnostic};
use std::{collections::HashMap, mem::MaybeUninit, str::FromStr};

pub fn part1(input: &str) -> miette::Result<usize> {
    let mut hands = Vec::new();
    for (num, line) in input.lines().enumerate() {
        let mut tokens = line.split_ascii_whitespace();
        let hand: Hand = tokens
            .next()
            .ok_or_else(|| miette!("missing hand at line {num}"))?
            .parse()?;
        let bid: usize = tokens
            .next()
            .ok_or_else(|| miette!("missing bid at line {num}"))?
            .parse()
            .into_diagnostic()?;
        hands.push((hand, bid));
    }
    hands.sort_unstable_by(|a, b| a.cmp(b).reverse());
    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum())
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut counts: Vec<_> = self
            .cards
            .iter()
            .fold(HashMap::<Card, u8>::new(), |mut acc, card| {
                *acc.entry(*card).or_default() += 1;
                acc
            })
            .into_iter()
            .collect();

        counts.sort_unstable_by(|a, b| a.1.cmp(&b.1).reverse());

        match counts[0].1 {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if counts[1].1 == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if counts[1].1 == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

impl FromStr for Hand {
    type Err = miette::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(miette!("string must have exactly 5 characters"));
        }
        let mut data: [MaybeUninit<Card>; 5] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..5 {
            data[i].write(s[i..i + 1].parse()?);
        }
        let cards = unsafe { std::mem::transmute::<_, [Card; 5]>(data) };
        Ok(Self { cards })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl FromStr for Card {
    type Err = miette::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Ace),
            "K" => Ok(Self::King),
            "Q" => Ok(Self::Queen),
            "J" => Ok(Self::Jack),
            "T" => Ok(Self::Ten),
            "9" => Ok(Self::Nine),
            "8" => Ok(Self::Eight),
            "7" => Ok(Self::Seven),
            "6" => Ok(Self::Six),
            "5" => Ok(Self::Five),
            "4" => Ok(Self::Four),
            "3" => Ok(Self::Three),
            "2" => Ok(Self::Two),
            _ => Err(miette!("invalid card symbol: {s}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Hand, HandType};

    #[test]
    fn hand_type() {
        let hand: Hand = "AAAAA".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::FiveOfAKind);
    }
}
