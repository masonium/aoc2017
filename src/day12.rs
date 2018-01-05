use prelude::*;

type Graph = HashMap<u32, Vec<u32>>;

fn bfs(g: &Graph) -> usize {
    let mut rem_nodes: HashSet<_> = g.keys().collect();
    let mut num_components = 0;

    while rem_nodes.len() > 0 {
        let s = *rem_nodes.iter().next().unwrap();
        let visited = bfs_component(g, *s);
        num_components += 1;
        for x in visited {
            rem_nodes.remove(&x);
        }
    }
    num_components
}

fn bfs_component(g: &Graph, start: u32) -> HashSet<u32> {
    let mut waiting: Vec<u32> = vec![start];
    let mut visited: HashSet<u32> = HashSet::new();

    while waiting.len() > 0 {
        let n = waiting.pop().unwrap();
        if !visited.contains(&n) {
            visited.insert(n);
            for x in &g[&n] {
                waiting.push(*x);
            }
        }
    }

    visited
}

fn read_graph<T: BufRead>(r: T) -> Graph {
    let mut g = Graph::new();
    let split_re = Regex::new(r"[- <>,]+").unwrap();
    for line in r.lines() {
        let l = line.unwrap();
        let tokens: Vec<_> = split_re.split(&l).collect();
        let n = tokens[0].parse::<u32>().unwrap();
        let neighbors: Vec<_> = tokens[1..]
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        g.insert(n, neighbors);
    }
    g
}

pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<usize> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);

    let g = read_graph(reader);


    Ok(match p {
        Part::A => bfs_component(&g, 0).len(),
        Part::B => bfs(&g)
    })
}
