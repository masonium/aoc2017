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
use aoc2017::day08::solve as day08_solve;
use aoc2017::day09::solve as day09_solve;
use aoc2017::day10::solve as day10_solve;
use aoc2017::day11::solve as day11_solve;
use aoc2017::day12::solve as day12_solve;
use aoc2017::day13::solve as day13_solve;
use aoc2017::day14::solve as day14_solve;
use aoc2017::day15::solve as day15_solve;
use aoc2017::day16::solve as day16_solve;
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

    println!("8A: {:?}", day08_solve("inputs/day08.txt", Part::A));
    println!("8B: {:?}", day08_solve("inputs/day08.txt", Part::B));

    println!("9A: {:?}", day09_solve("inputs/day09.txt", Part::A));
    println!("9B: {:?}", day09_solve("inputs/day09.txt", Part::B));

    let input_10 = "197,97,204,108,1,29,5,71,0,50,2,255,248,78,254,63";
    println!("10A: {:?}", day10_solve(input_10, Part::A));
    println!("10B: {:?}", day10_solve(input_10, Part::B));

    println!("11A: {:?}", day11_solve("inputs/day11.txt", Part::A));
    println!("11B: {:?}", day11_solve("inputs/day11.txt", Part::B));

    println!("12A: {:?}", day12_solve("inputs/day12.txt", Part::A));
    println!("12B: {:?}", day12_solve("inputs/day12.txt", Part::B));

    println!("13A: {:?}", day13_solve("inputs/day13.txt", Part::A));
    if !fast_only {
        println!("13B: {:?}", day13_solve("inputs/day13.txt", Part::B));
    }

    let input_14 = "jzgqcdpd";
    println!("14A: {:?}", day14_solve(input_14, Part::A));
    println!("14B: {:?}", day14_solve(input_14, Part::B));

    if !fast_only {
        println!("15A: {:?}", day15_solve(Part::A));
        println!("15B: {:?}", day15_solve(Part::B));
    }

    println!("16A: {:?}", day16_solve("inputs/day16.txt", Part::A));
    println!("16B: {:?}", day16_solve("inputs/day16.txt", Part::B));

}
