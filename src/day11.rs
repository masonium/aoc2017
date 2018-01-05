use prelude::*;
use std::cmp;

fn  hex_dir(d: &str) -> (i32, i32) {
    match d {
        "n" => (0, 2),
        "ne" => (1, 1),
        "se" => (1, -1),
        "s" => (0, -2),
        "sw" => (-1, -1),
        "nw" => (-1, 1),
        _ => panic!("bad direction")
    }
}

fn hex_dist(x: i32, y: i32) -> i32 {
    let ay = y.abs();
    let ax = x.abs();
    if ay > ax {
        ax + (ay-ax) / 2
    } else {
        ax
    }
}

pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<i32> {
    let file = File::open(f)?;
    let mut reader = BufReader::new(file);

    let mut steps = String::new();
    reader.read_line(&mut steps)?;

    let dirs: Vec<_> = steps.trim().split(",").map(|x| x.to_string()).collect();
    let pos = dirs.iter().fold((0, 0, 0), |a, dir| {
        let b = hex_dir(&dir);
        let new_pos = (a.0 + b.0, a.1 + b.1);
        let d = cmp::max(a.2, hex_dist(new_pos.0, new_pos.1));
        (new_pos.0, new_pos.1, d)
    });

    Ok(match p {
        Part::A => hex_dist(pos.0, pos.1),
        Part::B => pos.2
    })
}
