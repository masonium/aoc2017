use regex::Regex;
use std::convert::AsRef;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::fs::File;

use util::Part;

pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<u64> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"[ \t]+").unwrap();

    let mut checksum: u64 = 0;
    for line in reader.lines() {
        let mut nums: Vec<u64> = re.split(&line?).map(|s| {
            s.parse::<u64>().unwrap()
        }).collect();
        match p {
            Part::A => {
                checksum += nums.iter().max().unwrap() - nums.iter().min().unwrap();
            },
            Part::B => {
                nums.sort();
                for i in 0..nums.len() {
                    for j in i+1..nums.len() {
                        if nums[j] % nums[i] == 0 {
                            checksum += nums[j] / nums[i];
                            break;
                        }
                    }
                }
            }
        };
    }

    Ok(checksum)
}
