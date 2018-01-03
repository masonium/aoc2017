use prelude::*;

pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<u64> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map(|line| {
        let mut hs = HashSet::new();
        if let Ok(ref l) = line {
            for tok in l.split(' ') {
                let cmp_token = if p == Part::A {
                    tok.to_string()
                } else {
                    let mut r: Vec<u8> = tok.bytes().collect();
                    r.sort();
                    String::from_utf8(r).unwrap()
                };

                if hs.contains(&cmp_token) {
                    return 0;
                }
                hs.insert(cmp_token);
            }
            return 1;
        } else {
            return 0;
        }
    }).sum())
}
