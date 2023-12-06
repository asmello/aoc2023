# aoc2023
[![Nix CI](https://github.com/asmello/aoc2023/actions/workflows/ci.yml/badge.svg)](https://github.com/asmello/aoc2023/actions/workflows/ci.yml)

My solutions to Advent of Code 2023

## Goals

- Experiment with some new tooling, language features and crates.
- Test different approaches and design trade-offs.
- Improve how I approach problem-solving.
- Have some fun.

## Non-goals

- Rank high in the scoreboards. I'm not doing this competitively.
- Write the fastest, cleverest, most idiomatic or most concise code.
- Make optimal choices. I want to learn, including from mistakes.

## How this project is organized

I'm developing this as a library, with each day implemented as a module.

```
aoc2023
├── src
│  ├── lib.rs
│  ├── day1.rs
│  ├── day2.rs
│  └── day2
│     ├── chumsky.rs
│     └── manual.rs
└── tests
   ├── common
   │  └── mod.rs
   ├── day1.rs
   ├── day2.rs
   └── resources
      ├── day1
      │  └── input.txt
      └── day2
         └── input.txt
```

For the most part, each module exports a couple public functions `part1()` and `part2()` that implement the solutions to each part of the day's challenge. However, for day 6, I decided to treat the second part as having a different input, so the solution could remain the same - because of this, it exports `solve()` instead. I may go back on this if this turns out to be the only exception.

The argument types used by each module are also a bit inconsistent, sadly. This is because I initially wanted to have a very generic API that could take a file, memory buffer or anything implementing the `Read` trait. But after solving a few problems I decided to go back to using simple strings, as most inputs are tiny, and the boilerplate was getting annoying. So starting with day 5 you'll see `partX(input: &str)` as the typical interface to solvers.

Another inconsistency is that day 2, in particular, has two different implementations with two parsers. This is because I wanted to compare a manual parser with a generated one. I may add more multi-solution modules later, but I promise to always export a single "default" solution (as defined by being the one exported directly at the day module, and not in a submodule).

I'm implementing most tests as integration tests. Each day gets its own integration test module under `tests/`. This helps ensure I keep the public library interface adequate, as I'm effectively using it like an external consumer would (not that I expect any). Someone once told me a good (programming?) life hack is to always be your own client.

## Roadmap

- Complete the 2023 challenges
- Maybe add a CLI tool?
