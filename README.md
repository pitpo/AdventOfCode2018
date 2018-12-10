# Advent of Code 2018

This repository features all of my solutions for this year's AoC puzzles. 

This year I'm doing this in Rust, because I didn't have a lot of time to play with it after reading [The Book](https://doc.rust-lang.org/book/) and I really loved what I saw there. The initial solutions are most likely going to be ugly and dirty, but the plan is to refactor every single one of them (if needed) by the end of December. (UPDATE 12/10/18: As it turns out I'm usually uploading already cleaned-up solutions. There is nothing else to do early in the morning, so I'm refactorizing them just after getting correct answers) 

If you are wondering why did I go for seperate crate for each day, well I'm starting to wonder about that too. I guess saving a second per compilation on the last few puzzles is gonna be worth it? (UPDATE 12/10/18: No, no, no, hell no. This abomination of repository was a big mistake.)

The initial plan also includes solving at least some of puzzles in Haskell, because accomplishing anything in functional language is incredibly satisfying for me. 

Wish me luck :D

## Running solvers

Extract your session cookie and put it in "AOC_SESSION" environmental variable. Don't worry, AoC server is accessed only once per puzzle input. Alternatively you can just place your input files under `input/dayX.txt`. To run solver for day `X` type:

```
cargo run [--release] dayX
``` 

If you are planning to analyze my solutions, please keep in mind that I am not, by any means, an experienced programmer (or a remarkably smart person). There's always something that can be improved.