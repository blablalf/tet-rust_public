# Tetrust: a crash course to Rust

## Rationale

This is an attempt at coding a simple Tetris as a learning project for a Rust introduction course.

Our game must be as close as possible to the official spec (see `doc/`).  

## Rules

* Each course we discuss the specifications, what we should do and try to do it.
* We keep track of all the issues and the stuff we can't solve on the spot on a `changelog` at the root
* between each course we read the issues and try to code a solution.
* at the beginning of next course we look at the proposal that have been made, chose the best and keep going with the next step.

## Evaluation

* This represents half of your note for the whole course.
* Half is how you behave in class, other half is the track you leave on GH (issues, discussions, review, PR).
* You're allowed to share a repo with a mate, but each commit (or other contribution) must be individual (use your own account, make sure you commit under your name)

## tl;dr

* sign up on Github (if you're not already)
* Fork the repo in Github interface
* `git clone` to make a local copy
* `git checkout -b feature/<your_feature>` to start a new branch 
* code, commit, push
* open a pull request on Github interface (it should display a huge button for that)



## Personnal report
* I inspired myself a lot from https://www.youtube.com/watch?v=-JIlCYbpNnI

## Questions
* When am I able to just put a return value whitout the return keyword

## Bugs
* If you complete too many lines at once, there is an overflow bug -> Fixed
* You can't rotate the tetrimino one it is placed or if it is collisionning
* The game over appears to soon -> Fixed
* If you rotate the tetrimino at the last moment, it could possibly be badly placed