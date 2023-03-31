# Tetrust: Tetris in Rust

## Rationale

A simple Tetris in order to have fun with Rust.

Our game must be as close as possible to the official spec (see `doc/`).  

## Personnal report
* I inspired myself a lot from https://www.youtube.com/watch?v=-JIlCYbpNnI

## Questions
* When am I able to just put a return value whitout the return keyword

## Bugs
* If you complete too many lines at once, there is an overflow bug -> Fixed
* The game over appears to soon -> Fixed
* If you rotate the tetrimino at the last moment, it could possibly be badly placed // Looks like fixed ?! Maybe at the same time of the too soon game over bug
* Can't reproduce but I managed once to not be able to rotate the piece -> Looks like fixed
