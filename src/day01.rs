use std::io;
use std::convert::AsRef;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;

use util::Part;

pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<u64> {
    let f = File::open(f)?;
    let mut reader = BufReader::new(f);
    let mut buff = String::new();

    reader.read_line(&mut buff)?;
    let s = buff.trim();
    let b = s.as_bytes();

    let offset: usize = match p {
        Part::A => 1,
        Part::B => b.len() / 2
    };

    let sum =  (0..b.len()).map(|i| {
        if b[i] == b[(i + offset) % b.len()] {
            (b[i] - b'0') as u64
        } else {
            0
        }
    }).sum::<u64>();

    Ok(sum)
}
