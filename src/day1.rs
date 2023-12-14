use std::io::{BufRead, BufReader, Read};

use miette::{miette, Context, IntoDiagnostic};

pub fn part1(input: impl Read) -> miette::Result<u64> {
    let reader = BufReader::new(input);
    let mut sum = 0;
    for (num, line) in reader.lines().enumerate() {
        let line = line
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to read line {num}"))?;
        if line.is_empty() {
            continue;
        }
        let (mut first, mut last) = (None, None);
        for ch in line.chars() {
            if let Some(digit) = ch.to_digit(10) {
                first.get_or_insert(digit);
                last = digit.into();
            }
        }
        let first = first.ok_or_else(|| miette!("no valid digits in line {num}"))?;
        let last = last.expect("if first is set, last should be too");
        sum += 10 * first as u64 + last as u64;
    }
    Ok(sum)
}

// this is dumb, but it's probably the only algorithm that works as intended
fn replace_in_order(s: &str) -> String {
    s.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}

pub fn part2(input: impl Read) -> miette::Result<u64> {
    let reader = BufReader::new(input);
    let mut sum = 0;
    for (num, line) in reader.lines().enumerate() {
        let line = line
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to read line {num}"))?;
        if line.is_empty() {
            continue;
        }
        let replaced = replace_in_order(&line);
        let (mut first, mut last) = (None, None);
        for ch in replaced.chars() {
            if let Some(digit) = ch.to_digit(10) {
                first.get_or_insert(digit);
                last = digit.into();
            }
        }
        let first = first.ok_or_else(|| miette!("no valid digits in line {num}"))?;
        let last = last.expect("if first is set, last should be too");
        sum += 10 * first as u64 + last as u64;
    }
    Ok(sum)
}
