use prelude::*;

#[derive(Clone, Copy)]
struct Inst {
    val: u8,
    dir: i8,
    st: u8
}

impl Inst {
    fn new(x: u8, d: &str, state: u8) -> Inst {
        let val = x;
        let dir: i8 = match d { "right" => 1, "left" => -1, _ => panic!("bad dir") };
        let st = state - b'A';
        Inst {val, dir, st}
    }
}

#[derive(Clone, Copy)]
struct State {
    opt: [Inst; 2]
}

impl State {
    fn new(x: Inst, y: Inst) -> State {
        State { opt: [x, y] }
    }
}

pub fn solve() -> usize {
    let states = [State::new(Inst::new(1, "right", b'B'), Inst::new(0, "left", b'F')),
                  State::new(Inst::new(0, "right", b'C'), Inst::new(0, "right", b'D')),
                  State::new(Inst::new(1, "left", b'D'), Inst::new(1, "right", b'E')),
                  State::new(Inst::new(0, "left", b'E'), Inst::new(0, "left", b'D')),
                  State::new(Inst::new(0, "right", b'A'), Inst::new(1, "right", b'C')),
                  State::new(Inst::new(1, "left", b'A'), Inst::new(1, "right", b'A'))];

    let mut tape: HashMap<isize, u8> = HashMap::new();
    let mut cur_state: u8 = 0;
    let mut cur_pos: isize = 0;

    for _ in 0..12994925 {
        let t = *tape.get(&cur_pos).unwrap_or(&0) as usize;
        let s = states[cur_state as usize].opt[t];
        *tape.entry(cur_pos).or_insert(0) = s.val;
        cur_pos += s.dir as isize;
        cur_state = s.st;
    }
    tape.values().filter(|x| **x == 1).count()
}
