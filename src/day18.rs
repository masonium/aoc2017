use prelude::*;

#[derive(Clone, Copy, Debug)]
enum Ref {
    Reg(u8),
    Val(i64)
}

impl Ref {
    fn parse(s: &str) -> Ref {
        let x = s.as_bytes();
        if b'a' <= x[0] && x[0] <= b'z' {
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
    Add(u8, Ref),
    Mul(u8, Ref),
    Mod(u8, Ref),
    Snd(Ref),
    Rcv(u8),
    Jgz(Ref, Ref)
}

impl Inst {
    fn parse(s: &str) -> Inst {
        let ss = s.to_string();
        let toks: Vec<_> = ss.split(' ').collect();
        match toks[0] {
            "snd" => Inst::Snd(Ref::parse(toks[1])),
            "rcv" => Inst::Rcv(Ref::parse_reg(toks[1])),
            "set" => Inst::Set(Ref::parse_reg(toks[1]),
                               Ref::parse(toks[2])),
            "mul" => Inst::Mul(Ref::parse_reg(toks[1]),
                               Ref::parse(toks[2])),
            "add" => Inst::Add(Ref::parse_reg(toks[1]),
                               Ref::parse(toks[2])),
            "mod" => Inst::Mod(Ref::parse_reg(toks[1]),
                               Ref::parse(toks[2])),
            "jgz" => Inst::Jgz(Ref::parse(toks[1]),
                               Ref::parse(toks[2])),
            _ => panic!("unrecognized instuction: {}", s)
        }
    }
}

struct VMa {
    pc: isize,
    regs: [i64; 26],
    sounds: Vec<i64>
}

impl VMa {
    fn new() -> VMa {
        VMa {pc: 0, regs: [0; 26], sounds: Vec::new()}
    }
    fn val(&self, r: Ref) -> i64 {
        match r {
            Ref::Reg(c) => self.regs[c as usize],
            Ref::Val(a) => a
        }
    }

    fn run(&mut self, insts: &[Inst]) {
        self.pc = 0;
        while self.pc >= 0 && self.pc < insts.len() as isize {
            let inst = insts[self.pc as usize];
            let (offset, rcv) = self.run_inst(inst);
            self.pc += offset;
            if rcv { break; }
        }
    }

    fn run_inst(&mut self, inst: Inst) -> (isize, bool) {
        use self::Inst::*;
        let mut pc_offset: isize = 1;
        match inst {
            Snd(r) => {
                let v = self.val(r);
                self.sounds.push(v)
            },
            Rcv(r) => if self.val(Ref::Reg(r)) != 0 {
                let freq = self.sounds[self.sounds.len()-1];
                self.regs[r as usize] = freq;
                return (1, true);
            },
            Set(x, y) => self.regs[x as usize] = self.val(y),
            Add(x, y) => self.regs[x as usize] += self.val(y),
            Mul(x, y) => self.regs[x as usize] *= self.val(y),
            Mod(x, y) => self.regs[x as usize] %= self.val(y),
            Jgz(x, y) => {
                if self.val(x) > 0 {
                    pc_offset = self.val(y) as isize
                }
            }
        }
        (pc_offset, false)
    }
}

struct VMb {
    pc: isize,
    regs: [i64; 26],
    mailbox: Vec<i64>,
    send_count: i64
}

impl VMb {
    fn new(prog: i64) -> VMb {
        let mut regs = [0 as i64; 26];
        regs[(b'p' - b'a') as usize] = prog;
        VMb {pc: 0, regs: regs,
             send_count: 0, mailbox: Vec::new()}
    }
    fn val(&self, r: Ref) -> i64 {
        match r {
            Ref::Reg(c) => self.regs[c as usize],
            Ref::Val(a) => a
        }
    }

    /// run until the program is done, or until we need to wait on a
    /// result.
    fn run(&mut self, mailbox: &mut Vec<i64>, insts: &[Inst]) -> bool {
        let mut ran_any: bool = false;
        while self.pc >= 0 && self.pc < insts.len() as isize {
            let inst = insts[self.pc as usize];
            let (offset, suc) = self.run_inst(inst, mailbox);
            self.pc += offset;
            if suc {
                ran_any = true;
            } else {
                break;
            }
        }
        ran_any
    }

    /// Return the pc-offset, and return true iff the command was successful.
    fn run_inst(&mut self, inst: Inst, mailbox: &mut Vec<i64>) -> (isize, bool) {
        use self::Inst::*;
        let mut pc_offset: isize = 1;
        match inst {
            Snd(r) => {
                let v = self.val(r);
                self.mailbox.push(v);
                self.send_count += 1;
            },
            Rcv(r) => {
                if mailbox.len() > 0 {
                    let freq = mailbox.remove(0);
                    self.regs[r as usize] = freq;
                } else {
                    return (0, false);
                }
            },
            Set(x, y) => self.regs[x as usize] = self.val(y),
            Add(x, y) => self.regs[x as usize] += self.val(y),
            Mul(x, y) => self.regs[x as usize] *= self.val(y),
            Mod(x, y) => self.regs[x as usize] %= self.val(y),
            Jgz(x, y) => {
                if self.val(x) > 0 {
                    pc_offset = self.val(y) as isize
                }
            }
        }
        (pc_offset, true)
    }
}


pub fn solve<T: AsRef<Path>>(f: T, p: Part) -> io::Result<i64> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);

    let insts: Vec<_> = reader.lines().map(|s| Inst::parse(&s.unwrap())).collect();
    match p {
        Part::A => {
            let mut vm = VMa::new();
            vm.run(&insts);

            Ok(vm.sounds[vm.sounds.len() - 1])
        },
        Part::B => {
            let mut vm0 = VMb::new(0);
            let mut vm1 = VMb::new(1);
            loop {
                let ra = vm0.run(&mut vm1.mailbox, &insts);
                let rb = vm1.run(&mut vm0.mailbox, &insts);
                if !(ra || rb) {
                    break;
                }
            }
            Ok(vm1.send_count)
        }
    }
}
