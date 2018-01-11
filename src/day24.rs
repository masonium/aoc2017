use prelude::*;

type Piece = (usize, usize);
type PieceMap = HashMap<usize, HashSet<Piece>>;

fn remove_piece(m: &mut PieceMap, piece: Piece, port: usize) -> usize {
    m.get_mut(&piece.0).unwrap().remove(&piece);
    m.get_mut(&piece.1).unwrap().remove(&piece);
    piece.1 + piece.0 - port
}

fn insert_piece(m: &mut PieceMap, piece: Piece) {
    m.entry(piece.0).or_insert(HashSet::new()).insert(piece);
    m.entry(piece.1).or_insert(HashSet::new()).insert(piece);
}


fn find_best_bridge(port: usize, pieces: &mut PieceMap, total: usize) -> usize {
    if !pieces.contains_key(&port) {
        return total;
    }

    let s = pieces[&port].clone();
    let mut cur_t = total;
    for p in s.iter() {
        let other_port = remove_piece(pieces, *p, port);
        cur_t = cmp::max(cur_t, find_best_bridge(other_port, pieces,
                                                 total + p.0 + p.1));
        insert_piece(pieces, *p);
    }

    cur_t
}

fn find_best_longest_bridge(port: usize, pieces: &mut PieceMap,
                            total: (usize, usize)) -> (usize, usize) {
    if !pieces.contains_key(&port) {
        return total;
    }

    let s = pieces[&port].clone();
    let mut cur_t = total;
    for p in s.iter() {
        let other_port = remove_piece(pieces, *p, port);
        cur_t = cmp::max(cur_t, find_best_longest_bridge(other_port, pieces,
                                                 (total.0 + 1, total.1 + p.0 + p.1)));
        insert_piece(pieces, *p);
    }

    cur_t
}


pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<usize> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);

    let mut pieces: PieceMap = PieceMap::new();
    for piece in reader.lines().map(|s| {
        let r = s.unwrap();
        let mut iter = r.split("/");
        (iter.next().unwrap().parse().unwrap(),
         iter.next().unwrap().parse().unwrap())
    }) {
        insert_piece(&mut pieces, piece);
    }

    match p {
        Part::A => Ok(find_best_bridge(0, &mut pieces, 0)),
        Part::B => Ok(find_best_longest_bridge(0, &mut pieces, (0, 0)).1)
    }
}
