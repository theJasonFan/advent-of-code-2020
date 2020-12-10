use aoc::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = read_lines(args.last().unwrap()).unwrap();

    let mut prog = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            prog.push(CodePoint::from(&line))
        }
    }

    let mut prog = Prog {
        prog,
        acc: 0,
        pointer: 0,
    };

    println!("{:?}", &prog.run());

    for i in 0..prog.prog.len() {
        prog.reset();
        let exit = prog.run_buggy(i);
        match exit {
            Exit::LOOP(_) => continue,
            Exit::EXIT(_) => {
                println!("{:?}", exit);
                break;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Prog {
    prog: Vec<CodePoint>,
    acc: isize,
    pointer: usize,
}

impl Prog {
    fn run(&mut self) -> Exit {
        loop {
            if self.pointer >= self.prog.len() {
                return Exit::EXIT(self.acc);
            }
            let code_point = &mut self.prog[self.pointer];
            if code_point.visited {
                return Exit::LOOP(self.acc);
            } else {
                match code_point.op {
                    Op::NOP => self.pointer += 1,
                    Op::JMP => self.pointer = ((self.pointer as isize) + code_point.value) as usize,
                    Op::ACC => {
                        self.pointer += 1;
                        self.acc += code_point.value
                    }
                }
            }
            code_point.visited = true;
        }
    }

    // Runs program with buggy i'th instruction
    fn run_buggy(&mut self, buggy_i: usize) -> Exit {
        loop {
            if self.pointer >= self.prog.len() {
                return Exit::EXIT(self.acc);
            }

            let code_point = &mut self.prog[self.pointer];
            if code_point.visited {
                return Exit::LOOP(self.acc);
            }
            let mut op = code_point.op;

            if self.pointer == buggy_i {
                op = match op {
                    Op::NOP => Op::JMP,
                    Op::JMP => Op::NOP,
                    Op::ACC => Op::ACC,
                };
            }

            match op {
                Op::NOP => self.pointer += 1,
                Op::JMP => self.pointer = ((self.pointer as isize) + code_point.value) as usize,
                Op::ACC => {
                    self.pointer += 1;
                    self.acc += code_point.value
                }
            }

            code_point.visited = true;
        }
    }

    fn reset(&mut self) {
        self.pointer = 0;
        self.acc = 0;
        for cp in self.prog.iter_mut() {
            cp.visited = false;
        }
    }
}

#[derive(Debug)]
enum Exit {
    LOOP(isize),
    EXIT(isize),
}

#[derive(Debug, Clone)]
struct CodePoint {
    op: Op,
    value: isize,
    visited: bool,
}

impl CodePoint {
    #[allow(clippy::single_char_pattern)]
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(" ").collect();
        CodePoint {
            op: op_from_str(parts[0]).unwrap(),
            value: parts[1].parse().unwrap(),
            visited: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    NOP,
    ACC,
    JMP,
}

fn op_from_str(s: &str) -> Option<Op> {
    match s {
        "nop" => Some(Op::NOP),
        "acc" => Some(Op::ACC),
        "jmp" => Some(Op::JMP),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_signed_str() {
        let s = "+1";
        let i: isize = s.parse().unwrap();
        assert_eq!(i, 1);

        let s = "-1";
        let i: isize = s.parse().unwrap();
        assert_eq!(i, -1);

        let s = "-0";
        let i: isize = s.parse().unwrap();
        assert_eq!(i, 0);
    }
}
