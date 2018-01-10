use prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged
}

impl State {
    fn next_b(&self, dir: (isize, isize)) -> (State, (isize, isize)) {
        use self::State::*;
        match *self {
            Clean => (Weakened, turn_left(dir)),
            Weakened => (Infected, dir),
            Infected => (Flagged, turn_right(dir)),
            Flagged => (Clean, (-dir.0, -dir.1))
        }
    }

    fn next_a(&self, dir: (isize, isize)) -> (State, (isize, isize)) {
        use self::State::*;
        match *self {
            Clean => (Infected, turn_left(dir)),
            Infected => (Clean, turn_right(dir)),
            _ => panic!("invalid value")
        }
    }
}

type Grid = HashMap<(isize, isize), State>;

fn read_map(f: &str) -> io::Result<(Grid, usize)> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);

    let mut dim = 0;
    let mut map = HashMap::new();
    for (i, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        dim = l.bytes().count();
        for (j, _) in l.bytes().enumerate().filter(|&(_, x)| x == b'#') {
            map.insert((i as isize, j as isize), State::Infected);
        }
    }
    Ok((map, dim / 2))
}

fn turn_right(d: (isize, isize)) -> (isize, isize) {
    let (r, c) = d;
    (c, -r)
}

fn turn_left(d: (isize, isize)) -> (isize, isize) {
    let (r, c) = d;
    (-c, r)
}


pub fn solve(f: &str, p: Part) -> io::Result<usize> {
    let (mut grid, start) = read_map(f)?;

    let mut node_pos = (start as isize, start as isize);
    let mut node_dir = (-1 as isize, 0 as isize);

    let num_iter = if p == Part::A { 10000 } else { 10000000 };

    let mut count_infected = 0;
    for _ in 0..num_iter {
        let s = grid.entry(node_pos).or_insert(State::Clean);
        let (new_state, new_dir) = if p == Part::A {
            s.next_a(node_dir)
        } else {
            s.next_b(node_dir)
        };

        if new_state == State::Infected {
            count_infected += 1;
        }

        *s = new_state;
        node_dir = new_dir;

        node_pos = (node_pos.0 + node_dir.0, node_pos.1 + node_dir.1);
    }

    Ok(count_infected)
}
