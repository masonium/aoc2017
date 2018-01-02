extern crate aoc2017;

use aoc2017::{day01, util};

use day01::solve as day01_solve;
use aoc2017::day02::solve as day02_solve;
use aoc2017::day03::solve as day03_solve;
use util::Part;

fn main() {
    println!("{:?}", day01_solve("inputs/day01.txt", Part::A));
    println!("{:?}", day01_solve("inputs/day01.txt", Part::B));
    println!("{:?}", day02_solve("inputs/day02.txt", Part::A));
    println!("{:?}", day02_solve("inputs/day02.txt", Part::B));
    println!("{:?}", day03_solve(Part::A));
    println!("{:?}", day03_solve(Part::B));
}
