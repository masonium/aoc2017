use util::Part;
use std::collections::HashMap;

pub fn solve(p: Part) -> u64 {
    let input = 289326;
    match p {
        Part::A => spiral_dist(input),
        Part::B => first_exceeds(input)
    }
}

pub fn spiral_dist(m: u64) -> u64 {
    if m == 1 {
        return 0;
    }

    let s = (m as f64).sqrt().ceil() as u64;
    let next_odd = if s % 2 == 0 { s + 1 } else { s };
    // let next_odd_sq = next_odd * next_odd;
    let hw: i64 = ((next_odd - 1) / 2) as i64;
    let r: i64 = (m - 1) as i64 % (next_odd - 1) as i64;

    (hw + (hw - r).abs()) as u64
}

fn neighbors(g: (isize, isize)) -> Vec<(isize, isize)> {
    let (x, y) = g;
    vec![(x-1, y-1), (x, y-1), (x+1, y-1),
         (x-1, y  ),           (x+1, y),
         (x-1, y+1), (x, y+1), (x+1, y+1)]
}

fn ring(level: isize) -> Vec<(isize, isize)> {
    let mut r = Vec::new();
    r.extend( (-level+1..level+1).map(|i| (level, i)) );
    r.extend( (-level..level).rev().map(|i| (i, level)) );
    r.extend( (-level..level).rev().map(|i| (-level, i)) );
    r.extend( (-level+1..level+1).map(|i| (i, -level)) );
    r
}

fn first_exceeds(input: u64) -> u64 {
    let mut m: HashMap<(isize, isize), u64> = HashMap::new();

    m.insert((0, 0), 1);
    let mut level = 1;
    loop {
        for r in ring(level) {
            let s = neighbors(r).iter().map(|x| *m.get(x).unwrap_or(&0)).sum();
            m.insert(r, s);
            if s > input {
                return s
            }
        }
        level += 1;
    }
}
