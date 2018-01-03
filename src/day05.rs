use prelude::*;

pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<u64> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);

    let mut jumps: Vec<isize> = reader.lines().map(|x| {
        x.unwrap().parse::<isize>().unwrap()
    }).collect();

    let mut idx: isize = 0;
    let mut count = 0;
    while idx >= 0 && idx < jumps.len() as isize {
        let old_idx = idx as usize;
        idx += jumps[idx as usize];
        if p == Part::A {
            jumps[old_idx] += 1;
        } else {
            if jumps[old_idx] >= 3 {
                jumps[old_idx] -= 1;
            } else {
                jumps[old_idx] += 1;
            }
        }
        count += 1
    }
    Ok(count)
}
