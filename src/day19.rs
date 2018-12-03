use prelude::*;
use ndarray::{Array, Array1, Array2, Axis};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Spot {
    Empty,
    Path,
    Letter(u8)
}

fn grid<T: BufRead>(r: T) -> Array2<Spot> {
    use self::Spot::*;
    let mut g: Vec<Array1<_>> = Vec::new();
    for line in r.lines() {
        g.push(line.unwrap().bytes().map(|b| {
            match b {
                b'|' | b'+' | b'-' => Path,
                b' ' => Empty,
                b => Letter(b)
            }
        }).collect());
    }

    let max_len = g.iter().map(|c| c.len()).max().unwrap();
    let n = g.len();
    let mut res = Array::from_elem((n, max_len), Empty);

    for i in 0..n {
        res.slice_mut(s![i, 0..g[i].len()]).assign(&g[i]);
    }
    res
}

fn at(g:& Array2<Spot>, pos: (isize, isize)) -> Spot {
    let dim = g.dim();
    if pos.0 >= 0 && pos.0 < dim.0 as isize &&
        pos.1 >= 0 && pos.1 < dim.1 as isize {
            g[[pos.0 as usize, pos.1 as usize]]
        } else {
            Spot::Empty
        }
}

/// assuming the current direction is bad, find a new direction
fn next_dir(g: &Array2<Spot>, cur_pos: (isize, isize),
            dir: (isize, isize)) -> Option<(isize, isize)> {
    for perp in &[(dir.1, dir.0), (-dir.1, -dir.0)] {
        let pos = (cur_pos.0 + perp.0, cur_pos.1 + perp. 1);
        if at(g, pos) != Spot::Empty {
            return Some(*perp);
        }
    }
    None
}

fn trace_path(g: &Array2<Spot>, p: Part) -> String {
    let mut letters: Vec<u8> = Vec::new();
    let mut dir = (1, 0);

    // find the start
    let s = g.index_axis(Axis(0), 0).iter().position(|c| *c == Spot::Path).unwrap();
    let mut pos: (isize, isize) = (0, s as isize);
    let mut count = 1;

    loop {
        // If we can move in the same direction, do it.
        let next_pos: (isize, isize) = ((pos.0 + dir.0) as isize,
                                        (pos.1 + dir.1) as isize);
        let moved_forward =
            match at(g, next_pos) {
                Spot::Empty => false,
                Spot::Path => true,
                Spot::Letter(a) => {
                    letters.push(a);
                    true
                }
            };

        if moved_forward {
            count += 1;
            pos = next_pos;
        } else {
            // otherwise, find the next direction.
            if let Some(new_dir) = next_dir(g, pos, dir) {
                dir = new_dir;
            } else {
                break;
            }
        }
    }
    match p {
        Part::A => String::from_utf8(letters).unwrap(),
        Part::B => format!("{}", count)
    }
}

pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<String> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);

    let g = grid(reader);
    Ok(trace_path(&g, p))
}
