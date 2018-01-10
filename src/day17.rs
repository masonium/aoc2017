use prelude::*;

fn solve_spinlock(steps: usize, n: usize, val_after: usize) -> usize {
    let mut v: Vec<usize> = vec![0];
    let mut pos = 0;
    for i in 1..(n+1) {
        pos = (pos + steps) % i;
        v.insert(pos + 1, i);
        pos += 1;
    }

    let t = v.iter().position(|c| *c == val_after).unwrap();
    v[(t+1) % v.len()]
}

fn solve_sim_spinlock(steps: usize, n: usize) -> usize {
    let mut pos = 0;
    let mut ans = None;
    for i in 1..(n+1) {
        pos = (pos + steps) % i;
        pos += 1;
        if pos == 1 {
            ans = Some(i)
        }
    }
    ans.unwrap()
}


pub fn solve(p: Part) -> usize {
    match p {
        Part::A => solve_spinlock(335, 2017, 2017),
        Part::B => solve_sim_spinlock(335, 50000000)
    }
}
