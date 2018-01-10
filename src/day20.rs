use prelude::*;
use std::ops::Sub;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct V {

    x: isize,
    y: isize,
    z: isize
}

impl V {
    fn new(x: isize, y: isize, z: isize) -> V {
        V { x: x, y: y, z: z }
    }
}

impl Sub<V> for V {
    type Output = V;
    fn sub(self, v: V) -> V {
        V {x: self.x - v.x, y: self.y - v.y, z: self.z - v.z}
    }
}


struct Particle {
    pos: V,
    vel: V,
    acc: V
}

impl Particle {

    /// Returns true if two particles will collide at an integer point
    /// in time.
    pub fn collides_with(&self, other: &Particle) -> Option<isize> {
        let p = self.pos - other.pos;
        let v = self.vel - other.vel;
        let ac = self.acc - other.acc;

        let c = p.x as f64;
        let b = v.x as f64 + ac.x as f64 * 0.5;
        let a = ac.x as f64 * 0.5;

        let disc = b*b - 4.0 * a * c;
        let ts = if a == 0.0 {
            let t = -c / b;
            if t.round() == t && t >= 1.0 {
                vec![t as isize]
            } else {
                return None;
            }
        } else {
            if disc < 0.0 {
                return None;
            }
            let t1 = (-b - disc.sqrt()) / (2.0 * a);
            let t2 = (-b + disc.sqrt()) / (2.0 * a);

            let mut ts = Vec::new();
            for t in &[t1, t2] {
                if t.round() == *t && *t >= 1.0 {
                    ts.push(*t as isize);
                }
            }
            ts
        };

        for t in &ts {
            if p.y + v.y * t + t * (t + 1) * ac.y / 2 == 0 &&
                p.z + v.z * t + t * (t + 1) * ac.z / 2 == 0 {
                    return Some(*t);
                }
        }
        None
    }
}

fn parse_vector(line: &str) -> V {
    lazy_static! {
        static ref VECTOR_RE: Regex = Regex::new("[a-z]=<(?P<x>[-0-9]+),(?P<y>[-0-9]+),(?P<z>[-0-9]+)>").unwrap();
    }
    let cap = VECTOR_RE.captures(line).unwrap();
    V::new(cap["x"].parse().unwrap(),
           cap["y"].parse().unwrap(),
           cap["z"].parse().unwrap())
}

fn parse_particle(line: &str) -> Particle {
    let parts: Vec<_> = line.split(", ").collect();
    Particle { pos: parse_vector(parts[0]),
               vel: parse_vector(parts[1]),
               acc: parse_vector(parts[2]) }
}

pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<usize> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);

    let particles: Vec<_> = reader.lines()
        .map(|s| parse_particle(&s.unwrap()))
        .collect();

    match p {
        Part::A =>
            Ok(particles.iter().enumerate().min_by_key(|&(_, ref p)| {
                let a = p.acc;
                a.x.abs() + a.y.abs() + a.z.abs()
            }).unwrap().0),
        Part::B => {
            let mut s = HashSet::new();
            'outer: for i in 0..particles.len() {
                for j in (i+1)..particles.len() {
                    if s.contains(&j) {
                        continue
                    }
                    if let Some(_) = particles[i].collides_with(&particles[j]) {
                        s.insert(i);
                        s.insert(j);
                        continue 'outer;
                    }
                }
            }
            Ok(particles.len() - s.len())
        }
    }
}
