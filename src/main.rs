use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Eq, PartialEq)]
enum Opcode {
    Accumulate,
    Jump,
    NoOp,
    Invalid
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_rep = match self {
            Opcode::Accumulate => "Accumulate",
            Opcode::Jump => "Jump",
            Opcode::NoOp => "No Operation",
            Opcode::Invalid => "Invalid Operation"
        };
        write!(f, "{}", str_rep)
    }
}
#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    opcode: Opcode,
    value: isize,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.opcode, self.value)
    }
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_parts: Vec<&str> = s.split_whitespace().collect();
        let opcode = match s_parts[0] {
            "jmp" => Opcode::Jump,
            "acc" => Opcode::Accumulate,
            "nop" => Opcode::NoOp,
            _ => Opcode::Invalid
        };

        let value = s_parts[1].parse::<isize>()?;

        Ok(Instruction{ opcode, value })
    }
}

fn main() {
    let program: Vec<Instruction> = include_str!("input")
        .split('\n')
        .map(|inst| Instruction::from_str(inst).unwrap())
        .collect();

    let mut exec_path: Vec<usize> = Default::default();
    let mut acc: isize = 0;
    let mut ip: usize = 0;

    while !(exec_path.contains(&ip)) {
        let inst = &program[ip];
        exec_path.push(ip);
        match inst.opcode {
            Opcode::NoOp => ip += 1,
            Opcode::Jump => ip  = (ip as isize + inst.value) as usize,
            Opcode::Accumulate => { acc += inst.value; ip += 1 },
            Opcode::Invalid => panic!("Invalid OpCode")
        }
        if ip > program.len() {
            dbg!(exec_path);
            dbg!(&program[ip], ip, acc);
            break
        }
    }

}
