use prelude::*;

#[derive(Clone)]
struct Node {
    name: String,
    weight: usize,
    children: Vec<Rc<RefCell<Box<Node>>>>
}

#[derive(Clone, Debug)]
pub enum Answer {
    A(String),
    B(usize)
}

impl Node {
    fn from_file(f: &Path) -> io::Result<Rc<RefCell<Box<Node>>>> {
        let split_re = Regex::new(r"[ \t,()]+").unwrap();

        let file = File::open(f)?;
        let reader = BufReader::new(file);

        let c: Vec<_> = reader.lines().map(|line| {
            let x = line.unwrap().trim().to_string();
            let mut tokens: Vec<_> = split_re.split(&x).collect();
            let name = tokens[0].to_string();
            let weight = tokens[1].parse::<usize>().unwrap();
            let children = if tokens.len() > 2 {
                let c = tokens.split_off(3);
                c.iter().map(|x| x.to_string()).collect()
            } else {
                Vec::new()
            };
            (name.to_string(), (Rc::new(RefCell::new(Box::new(Node { name: name.to_string(), weight: weight, children: Vec::new() }))),
                                children))
        }).collect();

        let mut map: HashMap<_, _> = c.iter().cloned().collect();
        let mut parent_map = HashMap::new();
        let mut root = None;

        for (k, v) in map.iter() {
            let &(ref node, ref cnames) = v;
            if root == None {
                root = Some(k.to_string())
            }
            for child in cnames {
                node.borrow_mut().children.push(map[&child.to_string()].0.clone());
                parent_map.insert(child.to_string(), k.to_string());
            }
        }
        while let Some(p) = parent_map.get(root.as_ref().unwrap()) {
            root = Some(p.to_string());
        }


        Ok(map.remove(&root.unwrap()).unwrap().0)
    }

    pub fn total_weight(&self) -> usize {
        self.weight + self.children.iter().map(|rc| rc.borrow().total_weight()).sum::<usize>()
    }

    pub fn find_correct_weight(&self, target: Option<usize>) -> usize {
        // If there are no children, the leaf must be the target.
        if self.children.len() == 0 {
            return target.unwrap();
        }

        // Assume a solution exists.
        assert!(self.children.len() >= 2);
        let tws: Vec<_> = self.children.iter().map(|rc| rc.borrow().total_weight()).collect();
        let mode = if tws.len() <= 2 || tws[0] == tws[1] || tws[0] == tws[2] {
            tws[0]
        } else {
            tws[1]
        };
        let odd_out_i = tws.iter().position(|x| {*x != mode});

        match target {
            // if this is the root, compute the mode.
            None => {
                self.children[odd_out_i.unwrap()].borrow().find_correct_weight(Some(mode))
            },
            Some(t) => {
                match odd_out_i {
                    Some(o) => self.children[o].borrow().find_correct_weight(Some(mode)),
                    None => t - mode * self.children.len()
                }
            }
        }
    }
}

pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<Answer> {
    let n = Node::from_file(f.as_ref())?;
    Ok(match p {
        Part::A => Answer::A(n.borrow().name.clone()),
        Part::B => Answer::B(n.borrow().find_correct_weight(None))
    })
}
