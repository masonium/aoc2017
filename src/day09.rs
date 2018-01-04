use prelude::*;

fn parse(s: &[u8], p: Part, idx: usize, level: u32) -> (usize, u32) {
    if s[idx] == b'{' {
        parse_group(s, p, idx+1, level+1)
    } else if s[idx] == b'<' {
        parse_garbage(s, p, idx+1)
    } else {
        panic!("Incorrect character found: {} @ {}", s[idx] as char, idx);
    }
}

fn parse_group(s: &[u8], p: Part, mut idx: usize, level: u32) -> (usize, u32) {
    let mut group_total: u32 = 0;
    while s[idx] != b'}' {
        let x = parse(s, p, idx, level);
        group_total += x.1;
        idx = x.0;
        if s[idx] == b',' {
            idx += 1;
        } else if s[idx] != b'}' {
            panic!("Parse fail: expected '}}' instead of '{}'", s[idx]);
        }
    }
    (idx+1, match p {
        Part::A => level + group_total,
        Part::B => group_total
    })
}

fn parse_garbage(s: &[u8], p: Part, mut idx: usize) -> (usize, u32) {
    let mut ncc = 0;
    while s[idx] != b'>' {
        if s[idx] == b'!' {
            idx += 2;
        } else {
            idx += 1;
            ncc += 1;
        }
    }
    (idx+1, match p {
        Part::A => 0,
        Part::B => ncc
    })
}

pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<u32> {
    let file = File::open(f)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    Ok(parse(line.as_bytes(), p, 0, 0).1)
}
