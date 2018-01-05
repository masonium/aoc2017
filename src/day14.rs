use prelude::*;
use day10::knot_hash;

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
