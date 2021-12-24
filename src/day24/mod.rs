pub fn day24() {
    println!("Result 24-1: {}", part1());
    println!("Result 24-2: {}", part2());
}

pub fn part1() -> usize {
    let instructions = instructions();
    let mut registers = vec![(0_usize, [0, 0, 0, 0])];
    instructions.into_iter().for_each(|ins| {
        registers = registers
            .iter()
            .flat_map(|(lowest, r)| {
                (1..=9)
                    .map(|w| {
                        let mut r = *r;
                        r[W] = w;
                        let mut alu = ALU { registers: r };
                        ins.iter()
                            .for_each(|instruction| alu.evaluate(*instruction));
                        (lowest * 10 + w as usize, alu.registers)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        registers.sort_unstable_by(|a, b| match a.1.partial_cmp(&b.1).unwrap() {
            std::cmp::Ordering::Equal => b.0.partial_cmp(&a.0).unwrap(),
            ordering => ordering,
        });
        registers.dedup_by_key(|a| a.1);
    });
    registers
        .iter()
        .filter_map(|a| if a.1[Z] == 0 { Some(a.0) } else { None })
        .max()
        .unwrap()
}

pub fn part2() -> usize {
    let instructions = instructions();
    let mut registers = vec![(0_usize, [0, 0, 0, 0])];
    instructions.into_iter().for_each(|ins| {
        registers = registers
            .iter()
            .flat_map(|(lowest, r)| {
                (1..=9)
                    .map(|w| {
                        let mut r = *r;
                        r[W] = w;
                        let mut alu = ALU { registers: r };
                        ins.iter()
                            .for_each(|instruction| alu.evaluate(*instruction));
                        (lowest * 10 + w as usize, alu.registers)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        registers.sort_unstable_by(|a, b| match a.1.partial_cmp(&b.1).unwrap() {
            std::cmp::Ordering::Equal => a.0.partial_cmp(&b.0).unwrap(),
            ordering => ordering,
        });
        registers.dedup_by_key(|a| a.1);
    });
    registers
        .iter()
        .filter_map(|a| if a.1[Z] == 0 { Some(a.0) } else { None })
        .min()
        .unwrap()
}

const W: usize = 0;
const X: usize = 1;
const Y: usize = 2;
const Z: usize = 3;

struct ALU {
    registers: [i32; 4],
}

impl ALU {
    pub fn evaluate(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add(reg, input) => {
                self.registers[reg] = match input {
                    Input::Literal(value) => self.registers[reg] + value,
                    Input::Register(reg2) => self.registers[reg] + self.registers[reg2],
                }
            }
            Instruction::Mul(reg, input) => {
                self.registers[reg] = match input {
                    Input::Literal(value) => self.registers[reg] * value,
                    Input::Register(reg2) => self.registers[reg] * self.registers[reg2],
                }
            }
            Instruction::Div(reg, input) => {
                self.registers[reg] = match input {
                    Input::Literal(value) => self.registers[reg] / value,
                    Input::Register(reg2) => self.registers[reg] / self.registers[reg2],
                }
            }
            Instruction::Mod(reg, input) => {
                self.registers[reg] = match input {
                    Input::Literal(value) => self.registers[reg] % value,
                    Input::Register(reg2) => self.registers[reg] % self.registers[reg2],
                }
            }
            Instruction::Eql(reg, input) => {
                let equals = match input {
                    Input::Literal(value) => self.registers[reg] == value,
                    Input::Register(reg2) => self.registers[reg] == self.registers[reg2],
                };
                self.registers[reg] = if equals { 1 } else { 0 }
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Add(usize, Input),
    Mul(usize, Input),
    Div(usize, Input),
    Mod(usize, Input),
    Eql(usize, Input),
}

#[derive(Copy, Clone)]
enum Input {
    Literal(i32),
    Register(usize),
}

fn instructions() -> Vec<Vec<Instruction>> {
    let mut output = vec![vec![]; 14];
    let mut index = -1;
    raw_input().lines().for_each(|line| {
        let parts = line.split(' ').collect::<Vec<_>>();
        let input_1 = match parts[1] {
            "w" => W,
            "x" => X,
            "y" => Y,
            "z" => Z,
            _ => panic!("Unknown register"),
        };
        let input_2 = parts.get(2).map(|i| match *i {
            "w" => Input::Register(W),
            "x" => Input::Register(X),
            "y" => Input::Register(Y),
            "z" => Input::Register(Z),
            s => Input::Literal(s.parse::<i32>().expect("Unknown input")),
        });
        let instruction = match parts[0] {
            "inp" => {
                index += 1;
                None
            }
            "add" => Some(Instruction::Add(input_1, input_2.unwrap())),
            "mul" => Some(Instruction::Mul(input_1, input_2.unwrap())),
            "div" => Some(Instruction::Div(input_1, input_2.unwrap())),
            "mod" => Some(Instruction::Mod(input_1, input_2.unwrap())),
            "eql" => Some(Instruction::Eql(input_1, input_2.unwrap())),
            _ => panic!("Unknown instruction"),
        };
        if let Some(instruction) = instruction {
            output[index as usize].push(instruction);
        }
    });
    output
}

#[cfg(not(test))]
fn raw_input() -> &'static str {
    include_str!("input")
}

#[cfg(test)]
fn raw_input() -> &'static str {
    include_str!("testinput")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(99999999999999, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(11111111111111, part2());
    }
}
