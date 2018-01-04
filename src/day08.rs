use prelude::*;

fn test(a: i32, cmp: &str, b: i32) -> bool {
    match cmp {
        "==" => a == b,
        "<" => a < b,
        ">" => a > b,
        "<=" => a <= b,
        ">=" => a >= b,
        "!=" => a != b,
        _ => panic!("Invalid comparator: {}", cmp)
    }
}

fn offset(op: &str, val: i32) -> i32 {
    match op {
        "inc" => val,
        "dec" => -val,
        _ => panic!("Invalid op: {}", op)
    }
}

pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<i32> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);

    let mut m: HashMap<String, i32> = HashMap::new();

    let split_re = Regex::new(r"[ ]+").unwrap();
    let mut max_attained = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        let toks: Vec<_> = split_re.split(&l).collect();

        let cond = {
            let cmp_var = toks[4];
            let zero = 0;
            let cmp_val = m.get(cmp_var).unwrap_or(&zero);
            let tval = toks[6].parse::<i32>().unwrap();
            test(*cmp_val, toks[5], tval)
        };
        if cond {
            let var = toks[0].to_string();

            let o = offset(toks[1], toks[2].parse::<i32>().unwrap());
            let reg = m.entry(var).or_insert(0);
            *reg += o;
            if max_attained < *reg {
                max_attained = *reg;
            }
        }
    }
    Ok(match p {
        Part::A => *m.values().max().unwrap(),
        Part::B => max_attained
    })
}
