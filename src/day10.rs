use prelude::*;

#[derive(Debug)]
pub enum Answer {
    A(u16),
    B(String)
}

fn reverse_section(x: &mut [u8], start: usize, len: usize) {
    let l = x.len();
    for i in 0..len/2 {
        x.swap((start + i) % l, (start + len - 1 - i) % l);
    }
}

fn knot_mix(list: &mut [u8], lengths: &[usize], rounds: usize) {
    let mut skip_size = 0;
    let mut idx = 0;

    for _ in 0..rounds {
        for l in lengths {
            reverse_section(list, idx, *l);
            idx += *l as usize + skip_size;
            idx %= list.len();
            skip_size += 1;
        }
    }
}

pub fn knot_hash(line: &str) -> String{
    let mut list: Vec<u8> = (0..256).map(|x| x as u8).collect();
    let mut lengths: Vec<_> = line.bytes().map(|x| x as usize).collect();
    lengths.extend_from_slice(&[17,31,73,47,23]);
    knot_mix(&mut list, &lengths, 64);

    let vals: Vec<_> = list.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |a, b| a ^ b))
        .map(|x| format!("{:02x}", x)).collect();
    vals.join("")
}

pub fn solve(line: &str, p: Part) -> io::Result<Answer> {
    let split_re = Regex::new(r",").unwrap();

    let mut list: Vec<u8> = (0..256).map(|x| x as u8).collect();

    match p {
        Part::A => {
            let lengths: Vec<_> = split_re.split(line.trim()).map(|x| {
                x.parse::<usize>().unwrap()
            }).collect();

            knot_mix(&mut list, &lengths, 1);
            Ok(Answer::A(list[0] as u16 * list[1] as u16))
        },
        Part::B => {
            Ok(Answer::B(knot_hash(line)))
        }
    }
}
