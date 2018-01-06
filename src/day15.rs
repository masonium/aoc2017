use prelude::*;

const MOD: usize = 2147483647;

struct Generator {
    factor: usize,
    state: usize,
    tmod: usize
}

impl Generator {
    fn next(&mut self) -> usize {
        loop {
            self.state = (self.state * self.factor) % MOD;
            if self.state % self.tmod == 0 {
                break;
            }
        }
        self.state % 65536
    }
}

pub fn solve(p: Part) -> io::Result<usize> {
    let params = match p {
        Part::A => [1, 1, 40000000],
        Part::B => [4, 8, 5000000]
    };

    let mut gens = [Generator {factor: 16807, state: 277, tmod: params[0]},
                    Generator{factor: 48271, state: 349, tmod: params[1]}];
    Ok((0..params[2]).filter_map( |_| {
        let x = gens[0].next();
        let y = gens[1].next();
        if x == y { Some(1) } else { None }
    }).sum())
}
