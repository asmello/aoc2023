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

In general each day module will export two functions, `part1()` and `part2()`, which implement the solutions to each part of the day's challenge. For now, I'm planning to have each take `impl Read` as the main argument representing the full input. Although I expect all problems will have a reasonably small input that can comfortably fit in a String, this generic interface is a good way to practice building a more realistic API. Note that some problems involve additional parameters that may also be passed to these functions.

Some modules may contain submodules with alternative implementations. Since I'm taking an educational approach to these challenges, it is sometimes useful to compare different implementations. When this happens I'll export one of the implementations at the day module. The other implementations will also be public.

I'm implementing most tests as integration tests. Each day gets its own integration test module under `tests/`. This helps ensure I keep the public library interface adequate, as I'm effectively using it like an external consumer would (not that I expect any). Someone once told me a good (programming?) life hack is to always be your own client.

## Roadmap

- Complete the 2023 challenges
- Maybe add a CLI tool?
