use prelude::*;
use day10::knot_hash;
use ndarray::prelude::*;

pub fn count_used(key: &str) -> usize {
    lazy_static! {
        static ref CHAR_TO_COUNT: HashMap<char, usize> =
            [('0', 0),
             ('1', 1),
             ('2', 1),
             ('3', 2),
             ('4', 1),
             ('5', 2),
             ('6', 2),
             ('7', 3),
             ('8', 1),
             ('9', 2),
             ('a', 2),
             ('b', 3),
             ('c', 2),
             ('d', 3),
             ('e', 3),
             ('f', 4)].iter().cloned().collect();
    }

    (0..128).map(|i| {
        let kh = knot_hash(&format!("{}-{}", key, i));
        kh.chars().map(|c| CHAR_TO_COUNT[&c]).sum::<usize>()
    }).sum::<usize>()
}

pub fn grid_from_key(key: &str) -> Array2<i32> {
    let hashes: Vec<_> = (0..128).map(|i| {
        knot_hash(&format!("{}-{}", key, i))
    }).collect();
    let full = hashes.join("");
    let v: Vec<_> = full.chars().map(|c| format!("{:04b}", c.to_digit(16).unwrap())).collect();
    let digits: Vec<i32> = v.join("").bytes().map(|c|  {
        match c {
            b'0' => 0,
            b'1' => 1,
            _ => panic!("")
        }
    }).collect();

    Array::from_shape_vec((128, 128), digits).unwrap()
}

fn find_root( hm: &mut HashMap<usize, usize>, b: usize) -> usize {
    let x = hm[&b];
    if x != b {
        let nr = find_root(hm, x);
        hm.insert(b, nr);
    }
    hm[&b]
}

fn make_union( hm: &mut HashMap<usize, usize>, a: usize, b: usize) {
    let ar = find_root(hm, a);
    let br = find_root(hm, b);
    hm.insert(ar, br);
}

pub fn solve(key: &str, p: Part) -> usize {
    let g = grid_from_key(key);
    match p {
        Part::A => { g.scalar_sum() as usize },
        Part::B => {
            let mut regions = Array::zeros((128, 128));
            let mut region_map: HashMap<usize, usize> = HashMap::new();
            for i in 0..128 {
                for j in 0..128 {
                    //regions[[i, j]] = g[[i, j]];
                    // make a new region,
                    if g[[i, j]] == 1 {
                        let rid = j * 128 + i + 1;
                        regions[[i, j]] = rid;
                        region_map.insert(rid, rid);
                        if i > 0 {
                            if g[[i-1, j]] == 1 {
                                make_union( &mut region_map, regions[[i-1, j]], regions[[i, j]] );
                            }
                        }
                        if j > 0 {
                            if g[[i, j-1]] == 1 {
                                make_union( &mut region_map, regions[[i, j-1]], regions[[i, j]] );
                            }
                        }
                    }
                }
            }
            let mut uniq_region: HashSet<usize> = HashSet::new();
            for i in 0..128 {
                for j in 0..128 {
                    if g[[i, j]] == 1{
                        uniq_region.insert(find_root(&mut region_map, regions[[i, j]]));
                    }
                }
            }
            uniq_region.len()
        }
    }
}
