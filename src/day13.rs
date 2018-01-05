use prelude::*;

fn caught(depths: &HashMap<i32, i32>, start: i32) -> bool {
    for (k, v) in depths.iter() {
        if (k + start) % (2 * (v - 1)) == 0 {
            return true;
        }
    }
    false
}


fn severity(depths: &HashMap<i32, i32>, start: i32) -> (i32, bool) {
    depths.iter().map(|(k, v)| {
        let z = 2*(v - 1);
        if (k+start) % z == 0 {
            (k * v, true)
        } else {
            (0, false)
        }
    }).fold((0, false), |a, b| (a.0 + b.0, a.1 || b.1 ))
}

pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<i32> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);

    let depth_map: HashMap<_, _> = reader.lines().map(|s| {
        let tokens: Vec<_> = s.unwrap().split(": ").map(|x| x.trim().parse::<i32>().unwrap()).collect();
        (tokens[0], tokens[1])
    }).collect();

    Ok(match p {
        Part::A => severity(&depth_map, 0).0,
        Part::B => {
            let mut delay = 0;
            while caught(&depth_map, delay) {
                delay += 1;
            }
            delay
        }
    })
}
