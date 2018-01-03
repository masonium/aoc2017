use prelude::*;

pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<u64> {
    let file = File::open(f)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let r = Regex::new(r"[ \t]+").unwrap();
    reader.read_line(&mut line)?;
    let banks: Vec<_> = r.split(&line.trim()).map(|x| {
        x.parse::<u8>().unwrap()
    }).collect();

    Ok(until_repeat(banks, p))
}

fn until_repeat(mut v: Vec<u8>, p: Part) -> u64 {
    let mut hs: HashMap<Vec<u8>, usize> = HashMap::new();

    hs.insert(v.clone(), 0);
    let n = v.len();
    let mut count = 0;
    loop {
        let (max_i, val) = v.iter().cloned().enumerate()
            .max_by_key(|&(a, b)| (b, -(a as isize))).unwrap();

        v[max_i] = 0;
        for i in 0..val {
            v[(max_i + 1 + i as usize) % n] += 1;
        }
        count += 1;
        if hs.contains_key(&v) {
            return match p {
                Part::A => count,
                Part::B => count - hs[&v] as u64
            }
        } else {
            hs.insert(v.clone(), count as usize);
        }

    }
    count
}
