use advent_of_code_2015::elements::*;
use std::collections::{HashMap, HashSet};
use std::fs;

struct Transmogrifier {
    replacements: HashMap<Option<Element>, Vec<Vec<Option<Element>>>>,
    reverse: HashMap<Vec<Option<Element>>, Option<Element>>,
}

impl Transmogrifier {
    fn new() -> Transmogrifier {
        Transmogrifier {
            replacements: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    fn add_replacement(&mut self, a: &str, b: &str) {
        let input = symbol_to_element(a);
        let output = Transmogrifier::to_elements(b);
        self.replacements.entry(input.clone())
            .or_insert(Vec::new())
            .push(output.clone());
            
        self.reverse.insert(output, input);
    }

    fn to_elements(molecule: &str) -> Vec<Option<Element>> {
        let mut elements = Vec::new();
        let mut s = String::new();

        for x in molecule.chars() {
            if x.is_uppercase() && !s.is_empty() {
                elements.push(symbol_to_element(&s));
                s.clear();
            }
            s.push(x);
        }

        /* Grab the last element. */
        elements.push(symbol_to_element(&s));

        elements
    }

    fn transmogrify(&self, molecule: &[Option<Element>]) -> HashSet<Vec<Option<Element>>> {
        let mut output = HashSet::new();

        for idx in 0..molecule.len() {
            match self.replacements.get(&molecule[idx]) {
                Some(possibilities) => {
                    for p in possibilities {
                        let mut new_molecule = molecule.to_vec();
                        new_molecule.splice(idx..idx+1, p.iter().cloned());
                        output.insert(new_molecule);
                    }
                },
                None => (),
            }
        }

        output
    }

    fn reduce(&self, molecule: &[Option<Element>]) -> Vec<Vec<Option<Element>>> {
        let mut output = Vec::new();

        for idx in 0..molecule.len() {
            for (k, v) in self.reverse.iter() {
                if molecule[idx..].starts_with(k) {
                    let mut new_molecule = molecule.to_vec();
                    new_molecule.splice(idx..idx+k.len(), [v.clone()].iter().cloned());
                    output.push(new_molecule);
                }
            }
        }

        output
    }

    fn reduce_greedy(&self, molecule: &[Option<Element>]) -> Option<usize> {
        if molecule == &[None] {
            return Some(0)
        }

        for idx in 0..molecule.len() {
            for (k, v) in self.reverse.iter() {
                if molecule[0..].starts_with(k) {
                    let mut new_molecule = molecule.to_vec();
                    new_molecule.splice(idx..idx+k.len(), [v.clone()].iter().cloned());
                    return match self.reduce_greedy(&new_molecule) {
                        Some(x) => Some(1 + x),
                        None => None,
                    }
                }
            }
        }
        None
    }

    fn reduce_greedy_rev(&self, molecule: &[Option<Element>]) -> Option<usize> {
        if molecule == &[None] {
            return Some(0)
        }

        let mut new_molecule = molecule.to_vec();

        for idx in (0..molecule.len()).rev() {
            for (k, v) in self.reverse.iter() {
                if molecule[idx..].starts_with(k) {
                    new_molecule.splice(idx..idx+k.len(), [v.clone()].iter().cloned());
                    return match self.reduce_greedy_rev(&new_molecule) {
                        Some(x) => Some(1 + x),
                        None => None,
                    }
                }
            }
        }
        None
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day19.txt";
    let filename_2 = "inputs/day19_2.txt";

    let transmogrifier = build_transmogrifier(&filename)?;
    let molecule = read_molecule(&filename_2)?;

    let first_answer = transmogrifier.transmogrify(&molecule).len();
    println!("The first answer is: {}", first_answer);

    /* Hoping that either one of the greedy reducers work. */
    let second_answer =
        match transmogrifier.reduce_greedy(&molecule) {
            Some(x) => x,
            None => transmogrifier.reduce_greedy_rev(&molecule).unwrap_or(0),
        };
    
    println!("The second answer is: {}", second_answer);
    
    Ok(())
}

fn build_transmogrifier(filename: &str) -> Result<Transmogrifier, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let mut transmogrifier = Transmogrifier::new();

    for line in contents.lines() {
        let mut iter = line.split(" => ");
        transmogrifier.add_replacement(iter.next().unwrap(), iter.next().unwrap());
    }

    Ok(transmogrifier)
}

fn read_molecule(filename: &str) -> Result<Vec<Option<Element>>, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    Ok(Transmogrifier::to_elements(contents.trim()))
}
