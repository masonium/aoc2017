use prelude::*;

#[derive(Clone, Copy, Debug)]
enum Ref {
    Reg(u8),
    Val(i64)
}

impl Ref {
    fn parse(s: &str) -> Ref {
        let x = s.as_bytes();
        if b'a' <= x[0] && x[0] <= b'h' {
            Ref::Reg((x[0] - b'a') as u8)
        } else {
            Ref::Val(s.parse().unwrap())
        }
    }

    fn parse_reg(s: &str) -> u8 {
        let x = s.as_bytes();
        (x[0] - b'a') as u8
    }
}

#[derive(Clone, Copy, Debug)]
enum Inst {
    Set(u8, Ref),
    Sub(u8, Ref),
    Add(u8, Ref),
    Mul(u8, Ref),
    Jnz(Ref, Ref)
}

impl Inst {
    fn parse(s: &str) -> Inst {
        let ss = s.to_string();
        let toks: Vec<_> = ss.split(' ').collect();
        match toks[0] {
            "set" => Inst::Set(Ref::parse_reg(toks[1]),
                               Ref::parse(toks[2])),
            "mul" => Inst::Mul(Ref::parse_reg(toks[1]),
                               Ref::parse(toks[2])),
            "sub" => Inst::Sub(Ref::parse_reg(toks[1]),
                               Ref::parse(toks[2])),
            "add" => Inst::Add(Ref::parse_reg(toks[1]),
                               Ref::parse(toks[2])),
            "jnz" => Inst::Jnz(Ref::parse(toks[1]),
                               Ref::parse(toks[2])),
            _ => panic!("unrecognized instuction: {}", s)
        }
    }
}

struct VMa {
    pc: isize,
    regs: [i64; 8],
    nd: bool
}

impl VMa {
    fn new(nd: bool) -> VMa {
        let mut regs = [0; 8];
        if nd {
            regs[0] = 1;
        }
        VMa {pc: 0, regs: regs, nd: nd}
    }
    fn val(&self, r: Ref) -> i64 {
        match r {
            Ref::Reg(c) => self.regs[c as usize],
            Ref::Val(a) => a
        }
    }

    fn run(&mut self, insts: &[Inst]) -> usize{
        self.pc = 0;
        let mut mul_count = 0;
        while self.pc >= 0 && self.pc < insts.len() as isize {
            let inst = insts[self.pc as usize];
            if let Inst::Mul(_, _) = inst {
                mul_count += 1;
            }
            let (offset, mut_d) = self.run_inst(inst);
            if mut_d && self.nd {
                println!("{:?}", self.regs);
            }

            self.pc += offset;
        }

        mul_count
    }

    fn run_inst(&mut self, inst: Inst) -> (isize, bool) {
        use self::Inst::*;
        let mut pc_offset: isize = 1;
        let mut mut_d = false;
        match inst {
            Set(x, y) => { mut_d = x == 3 || x == 4; self.regs[x as usize] = self.val(y); }
            Sub(x, y) => { mut_d = x == 3 || x == 4; self.regs[x as usize] -= self.val(y); }
            Add(x, y) => { mut_d = x == 3 || x == 4; self.regs[x as usize] += self.val(y); }
            Mul(x, y) => { mut_d = x == 3 || x == 4; self.regs[x as usize] *= self.val(y); }
            Jnz(x, y) => {
                if self.val(x) != 0 {
                    pc_offset = self.val(y) as isize
                }
            }
        }
        (pc_offset, mut_d)
    }
}

// pub fn is_prime(x) {

// }

pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<usize> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);

    let insts: Vec<_> = reader.lines().map(|s| Inst::parse(&s.unwrap())).collect();
    let mut vm = VMa::new(p == Part::B);

    Ok(vm.run(&insts))
}
