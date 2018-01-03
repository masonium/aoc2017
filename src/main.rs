extern crate aoc2017;
extern crate regex;

use regex::Regex;

use aoc2017::{day01, util};

use day01::solve as day01_solve;
use aoc2017::day02::solve as day02_solve;
use aoc2017::day03::solve as day03_solve;
use aoc2017::day04::solve as day04_solve;
use aoc2017::day05::solve as day05_solve;
use aoc2017::day06::solve as day06_solve;
use aoc2017::day07::solve as day07_solve;
use util::Part;

fn main() {
    let fast_only = true;

    println!("1A: {:?}", day01_solve("inputs/day01.txt", Part::A));
    println!("1B: {:?}", day01_solve("inputs/day01.txt", Part::B));
    println!("2A: {:?}", day02_solve("inputs/day02.txt", Part::A));
    println!("2B: {:?}", day02_solve("inputs/day02.txt", Part::B));
    println!("3A: {:?}", day03_solve(Part::A));
    println!("3B: {:?}", day03_solve(Part::B));
    println!("4A: {:?}", day04_solve("inputs/day04.txt", Part::A));
    println!("4B: {:?}", day04_solve("inputs/day04.txt", Part::B));
    if !fast_only {
        println!("5A: {:?}", day05_solve("inputs/day05.txt", Part::A));
        println!("5B: {:?}", day05_solve("inputs/day05.txt", Part::B));
    }
    println!("6A-Test: {:?}", day06_solve("inputs/day06_test.txt", Part::A));
    println!("6B-Test: {:?}", day06_solve("inputs/day06_test.txt", Part::B));
    println!("6A: {:?}", day06_solve("inputs/day06.txt", Part::A));
    println!("6B: {:?}", day06_solve("inputs/day06.txt", Part::B));

        let node_re = Regex::new(r"(?x)
(?P<name>[a-z]+)
\s+
\((?P<weight>[0-9]+)\)
").unwrap();

    node_re.captures("abcde (1)").unwrap();

    println!("7A: {:?}", day07_solve("inputs/day07.txt", Part::A));
    println!("7B: {:?}", day07_solve("inputs/day07.txt", Part::B));
}
