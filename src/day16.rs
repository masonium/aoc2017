use prelude::*;

#[derive(Clone, Copy, Debug)]
enum DM {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8)
}
impl DM {
    fn parse(s: &str) -> DM {
        lazy_static! {
            static ref SPIN_RE: Regex = Regex::new("s(?P<a>[0-9]+)").unwrap();
            static ref EXCHANGE_RE: Regex = Regex::new("x(?P<a>[0-9]+)/(?P<b>[0-9]+)").unwrap();
            static ref PARTNER_RE: Regex = Regex::new("p(?P<a>[a-p])/(?P<b>[a-p])").unwrap();
        }
        if let Some(cap) = SPIN_RE.captures(s) {
            DM::Spin(cap["a"].parse().unwrap())
        } else if let Some(cap) = EXCHANGE_RE.captures(s) {
            DM::Exchange(cap["a"].parse().unwrap(), cap["b"].parse().unwrap())
        } else if let Some(cap) = PARTNER_RE.captures(s) {
            DM::Partner(cap["a"].as_bytes()[0], cap["b"].as_bytes()[0])
        } else {
            panic!("bad parse: {}", s);
        }
    }
}

fn do_dance(progs: &mut Vec<u8>, dms: &[DM]) {
    for dm in dms {
        match *dm {
            DM::Spin(a) => { progs.rotate(16 - a); },
            DM::Exchange(a, b) => { progs.swap(a, b); },
            DM::Partner(x, y) => {
                let a = progs.iter().position(|c| *c == x).unwrap();
                let b = progs.iter().position(|c| *c == y).unwrap();
                progs.swap(a, b);
            }
        }
    }
}

pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<String> {
    let file = File::open(f)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    reader.read_line(&mut line)?;

    let dms: Vec<_> = line.split(",").map(|s| DM::parse(s)).collect();
    let mut progs: Vec<_> = (0..16).map(|x| b'a' + x).collect();
    let orig = progs.clone();

    match p {
        Part::A => {
            do_dance(&mut progs, &dms);
            Ok(String::from_utf8(progs).unwrap())
        },
        Part::B => {
            // find fixed_point
            let mut fp = 0;
            loop {
                do_dance(&mut progs, &dms);
                fp += 1;
                if orig == progs {
                    break;
                }
            }

            let rem = 1000000000 % fp;
            for _ in 0..rem {
                do_dance(&mut progs, &dms);
            }
            Ok(String::from_utf8(progs).unwrap())
        }
    }
}
