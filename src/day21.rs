use prelude::*;
use ndarray::{Array, Array2, arr2};

fn pattern(x: &str) -> u16 {
    pattern_bits(x).into_iter().fold(0, |x, y| 2 * x + y as u16)
}

fn as_mat(x: &str, n: usize) -> Array2<u8> {
    Array::from_iter(pattern_bits(x).into_iter()).into_shape((n, n)).unwrap()
}

pub fn pattern_bits(x: &str) -> Vec<u8> {
    x.bytes().filter_map(|x| {
        match x {
            b'.' => Some(0),
            b'#' => Some(1),
            _ => None
        }
    }).collect()
}

pub fn comb2(a: u8, b: u8, c: u8, d: u8) -> u16 {
    (a * 8 + b * 4 + c * 2 + d) as u16
}

pub fn rot2(x: u16) -> [u16; 4] {
    let a = (x / 8) as u8;
    let b = (x / 4 % 2) as u8;
    let c = (x / 2 % 2) as u8;
    let d = (x % 2) as u8;

    [comb2(a, b, c, d), comb2(c, a, d, b), comb2(d, c, b, a), comb2(b, d, a, c)]
}

pub fn break3(x: u8) -> (u8, u8, u8) {
    ((x / 4) as u8, ((x / 2) % 2) as u8, (x % 2) as u8)
}

pub fn c3(x: u8, y: u8, z: u8) -> u16 {
    (x * 4 + y * 2 + z) as u16
}
pub fn d3(x: u16, y: u16, z: u16) -> u16 {
    x * 64 + y * 8 + z
}

pub fn rot3(n: u16) -> [u16; 8] {
    let (a, b, c) = break3((n / 64) as u8);
    let (d, e, f) = break3((n / 8 % 8) as u8);
    let (g, h, i) = break3((n % 8) as u8);

    [d3(c3(a, b, c), c3(d, e, f), c3(g, h, i)),
     d3(c3(g, d, a), c3(h, e, b), c3(i, f, c)),
     d3(c3(i, h, g), c3(f, e, d), c3(c, b, a)),
     d3(c3(c, f, i), c3(b, e, h), c3(a, d, g)),
     d3(c3(g, h, i), c3(d, e, f), c3(a, b, c)),
     d3(c3(i, f, c), c3(h, e, b), c3(g, d, a)),
     d3(c3(c, b, a), c3(f, e, d), c3(i, h, g)),
     d3(c3(a, d, g), c3(b, e, h), c3(c, f, i))
    ]
}

type PMap = HashMap<u16, Array2<u8>>;

fn collect_patterns<T: BufRead>(reader: T) -> io::Result<(PMap, PMap)> {
    let mut p22 = HashMap::new();
    let mut p33 = HashMap::new();
    for line in reader.lines() {
        let l = line?;
        let tokens: Vec<_> = l.split(" => ").collect();
        match tokens[0].len() {
            5 => {
                let a = pattern(tokens[0]);
                let out = as_mat(tokens[1], 3);
                for x in &rot2(a) {
                    p22.insert(*x, out.clone());
                }
            },
            11 => {
                let a = pattern(tokens[0]);
                let out = as_mat(tokens[1], 4);
                for x in &rot3(a) {
                    p33.insert(*x, out.clone());
                }

            },
            _ => {
                return Err(
                    io::Error::new(io::ErrorKind::InvalidInput,
                                   format!("bad length {}", tokens[0].len())));
            }
        }
    }
    Ok((p22, p33))
}
pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<usize> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);

    let (p22, p33) = collect_patterns(reader)?;
    let mut curr = arr2(&[[0, 1, 0], [0, 0, 1], [1, 1, 1]]);
    let mut next;

    let num_iter = if p == Part::A { 5 } else { 18 };

    for _ in 0..num_iter {
        let d = curr.dim().0;
        let (m, new_size) = if d % 2 == 0 {
            (2, d * 3 / 2)
        } else {
            (3, d * 4 / 3)
        };
        let n = m + 1;
        next = Array2::zeros((new_size, new_size));
        let map = if m == 2 { &p22 } else { &p33 };
        for j in 0..(d/m) {
            for i in 0..(d/m) {
                let submat = curr.slice(s![i*m..(i+1)*m, j*m..(j+1)*m]);
                let x = submat.iter().fold(0, |x, y| 2 * x as u16 + *y as u16);
                next.slice_mut(s![i*n..(i+1)*n, j*n..(j+1)*n])
                    .assign(&map[&x]);
            }
        }
        curr = next;
    }
    Ok(curr.iter().filter(|x| **x == 1).count())
}
