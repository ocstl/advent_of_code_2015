use std::fs;

#[derive(Debug)]
enum JumpOffset {
    Positive(usize),
    Negative(usize),
}

impl JumpOffset {
    fn new(s: &str) -> JumpOffset {
        let size: usize = s[1..].parse().unwrap();
        
        match &s[0..1] {
            "+" => JumpOffset::Positive(size),
            "-" => JumpOffset::Negative(size),
            _ => panic!("Unreadable jump offset."),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Half(usize),
    Triple(usize),
    Increment(usize),
    Jump(JumpOffset),
    JumpIfEven(usize, JumpOffset),
    JumpIfOne(usize, JumpOffset),
}

#[derive(Debug)]
struct Computer {
    registers: [usize; 2],
    memory: Vec<Instruction>,
    instruction_pointer: usize,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            registers: [0; 2],
            memory: Vec::new(),
            instruction_pointer: 0
        }
    }

    fn set_register(&mut self, register: usize, value: usize) {
        self.registers[register] = value;
    }

    fn load_program(&mut self, filename: &str) -> Result<(), std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        for line in contents.lines() {
            self.memory.push(Computer::read_instruction(line));
        }

        Ok(())
    }

    fn read_instruction(instruction: &str) -> Instruction {
        let mut iter = instruction.split_whitespace();

        let to_register = |s: &str| -> usize {
            match s {
                "a" => 0,
                "b" => 1,
                _ => panic!("Invalid register."),
            }
        };

        match iter.next().unwrap() {
            "hlf" => Instruction::Half(to_register(iter.next().unwrap())),
            "tpl" => Instruction::Triple(to_register(iter.next().unwrap())),
            "inc" => Instruction::Increment(to_register(iter.next().unwrap())),
            "jmp" => {
                Instruction::Jump(JumpOffset::new(iter.next().unwrap()))
            },
            "jie" => {
                Instruction::JumpIfEven(
                    /* Watch out for the comma after the register name. */
                    to_register(&iter.next().unwrap()[0..1]),
                    JumpOffset::new(iter.next().unwrap())
                )
            },
            "jio" => {
                Instruction::JumpIfOne(
                    /* Watch out for the comma after the register name. */
                    to_register(&iter.next().unwrap()[0..1]),
                    JumpOffset::new(iter.next().unwrap())
                )
            },
            _ => panic!("Invalid instruction."),
        }
    }

    fn execute_program(&mut self) {
        while let Some(i) = self.memory.get(self.instruction_pointer) {
            match i {
                Instruction::Half(x) => {
                    self.registers[*x] /= 2;
                    self.instruction_pointer += 1;
                },
                Instruction::Triple(x) => {
                    self.registers[*x] *= 3;
                    self.instruction_pointer += 1;
                },
                Instruction::Increment(x) => {
                    self.registers[*x] += 1;
                    self.instruction_pointer += 1;
                },
                Instruction::Jump(x) => {
                    match x {
                        JumpOffset::Positive(y) =>
                            self.instruction_pointer += y,
                        JumpOffset::Negative(y) =>
                            self.instruction_pointer -= y,
                    }
                },
                Instruction::JumpIfEven(r, x) => {
                    if self.registers[*r] % 2 == 0 {
                        match x {
                            JumpOffset::Positive(y) =>
                            self.instruction_pointer += y,
                        JumpOffset::Negative(y) =>
                            self.instruction_pointer -= y,
                        }
                    } else {
                        self.instruction_pointer += 1;
                    }
                },
                Instruction::JumpIfOne(r, x) =>  {
                    if self.registers[*r] == 1 {
                        match x {
                            JumpOffset::Positive(y) =>
                                self.instruction_pointer += y,
                            JumpOffset::Negative(y) =>
                                self.instruction_pointer -= y,
                        }
                    } else {
                        self.instruction_pointer += 1;
                    }
                },
            }
        }   
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day23.txt";

    let mut computer = Computer::new();
    computer.load_program(&filename)?;
    computer.execute_program();
    println!("The first answer is: {}", computer.registers[1]);

    /* Reset for part 2. */
    let mut computer = Computer::new();
    computer.load_program(&filename)?;
    computer.set_register(0, 1);
    computer.execute_program();
    println!("The second answer is: {}", computer.registers[1]);
    
    Ok(())
}
