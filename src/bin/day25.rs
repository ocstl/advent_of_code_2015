use std::fs;

const MULTIPLIER: u64 = 252533;
const MODULUS: u64 = 33554393;
const SEED: u64 = 20151125;

#[derive(Clone)]
struct CodeGenerator {
    row: usize,
    column: usize,
    current: u64,
}

impl CodeGenerator {
    fn new() -> CodeGenerator {
        CodeGenerator { row: 1, column: 1, current: SEED }
    }
}

impl Iterator for CodeGenerator {
    type Item = CodeGenerator;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.row {
            1 => { self.row = self.column + 1; self.column = 1; },
            _ => { self.row -= 1; self.column += 1; },
        }
        
        self.current = self.current * MULTIPLIER % MODULUS;
        Some(self.clone())
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day25.txt";
    let contents = fs::read_to_string(&filename)?;
    let idx = contents.trim().find(|c: char| c.is_ascii_digit()).unwrap();
    
    let mut iter = contents[idx..].split_whitespace();
    let row: usize =
        iter.next().unwrap().trim_matches(|c| c == ',').parse().unwrap();
    let column: usize =
        iter.nth(1).unwrap().trim_matches(|c| c == '.').parse().unwrap();

    let mut code_generator = CodeGenerator::new();
    let first_answer =
        code_generator.find(|c| c.row == row && c.column == column)
            .unwrap()
            .current;

    println!("The first answer is: {}", first_answer);    
    
    Ok(())
}
