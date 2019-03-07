use permutohedron::Heap;
use std::collections::{HashMap, HashSet};
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day13.txt";
    let happinesses = read_input(&filename)?;
    let mut persons = happinesses.keys()
            .map(|k| k.0.clone())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect::<Vec<String>>();

    let first_answer = part1(&happinesses, &mut persons);
    println!("The first answer is: {}", first_answer);

    let second_answer = part2(&happinesses, &mut persons);
    println!("The second answer is: {}", second_answer);

    Ok(())
}

fn read_input(filename: &str) ->
        Result<HashMap<(String, String), i32>, std::io::Error> {

    let mut result = HashMap::new();

    fs::read_to_string(filename)?
        .lines()
        .for_each(|line| {
            let mut iter = line.split_whitespace();
            let person_a = iter.nth(0).unwrap();
            let gain_loss = match iter.nth(1).unwrap() {
                "gain" => iter.nth(0).unwrap().parse::<i32>().unwrap(),
                "lose" => -(iter.nth(0).unwrap().parse::<i32>().unwrap()),
                _ => panic!("Wrongly structured."),
            };
            let person_b = iter.last().unwrap().trim_matches('.');

            result.insert((person_a.to_string(), person_b.to_string()), gain_loss);
        });

    Ok(result)
}

fn part1(happinesses: &HashMap<(String, String), i32>, persons: &mut Vec<String>)
        -> i32 {
    
    Heap::new(persons).map(|p| {
        let person_a = p.first().unwrap();
        let person_b = p.last().unwrap();

        happinesses.get(&(person_a.to_string(), person_b.to_string())).unwrap()
            + happinesses.get(&(person_b.to_string(), person_a.to_string())).unwrap()
            + p.windows(2).map(|w| {
                let person_a = w.first().unwrap();
                let person_b = w.last().unwrap();
                happinesses.get(&(person_a.to_string(), person_b.to_string())).unwrap()
                    + happinesses.get(&(person_b.to_string(), person_a.to_string())).unwrap()
            }).sum::<i32>()
    }).max().unwrap()
}

fn part2(happinesses: &HashMap<(String, String), i32>, mut persons: &mut Vec<String>)
        -> i32 {

    /* Build new hashmap to add an apathetic (me!) person. */
    let new_person = "Apathetic";
    let mut happinesses = happinesses.clone();
    
    persons.iter()
        .for_each(|person| {
            happinesses.insert((new_person.to_string(), person.clone()), 0);
            happinesses.insert((person.clone(), new_person.to_string()), 0);
        });

    persons.push(new_person.to_string());

    part1(&happinesses, &mut persons)
}
