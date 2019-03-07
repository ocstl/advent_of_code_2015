extern crate permutohedron;

use permutohedron::Heap;
use std::collections::{HashSet, HashMap};
use std::fs;

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day9.txt";

    let distances = read_input(&filename)?;
    let cities: HashSet<String> = HashSet::from(
        distances.keys()
            .map(|k| k.get(0).unwrap().to_string())
            .collect());

    let first_answer = part1(&cities, &distances);
    println!("The first answer is: {}", first_answer);

    let second_answer = part2(&cities, &distances);
    println!("The second answer is: {}", second_answer);    
    
    Ok(())
}

fn part1(cities: &HashSet<String>, distances: &HashMap<[String; 2], u32>)
        -> u32 {
    let mut v: Vec<String> = cities.iter().map(|x| x.to_string()).collect();
    let heap = Heap::new(&mut v);

    heap.map(|path| {
            path.as_slice()
                .windows(2)
                .fold(0, |acc, p| acc + distances.get(p).unwrap())
            })
        .min()
        .unwrap()
}

fn part2(cities: &HashSet<String>, distances: &HashMap<[String; 2], u32>)
        -> u32 {
    let mut v: Vec<String> = cities.iter().map(|x| x.to_string()).collect();
    let heap = Heap::new(&mut v);

    heap.map(|path| {
            path.as_slice()
                .windows(2)
                .fold(0, |acc, p| acc + distances.get(p).unwrap())
            })
        .max()
        .unwrap()
}

fn read_input(filename: &str) ->
        Result<HashMap<[String; 2], u32>, std::io::Error> {

    let contents = fs::read_to_string(filename)?;

    let mut distances = HashMap::new();

    contents.lines()
        .for_each(|line| {
            let mut cities_distance = line.split(" = ");
            let cities: Vec<&str> =
                cities_distance.next().unwrap().split(" to ").collect();
            let distance = cities_distance.next().unwrap().parse().unwrap();
            distances.insert(
                [cities.get(0).unwrap().to_string(),
                cities.get(1).unwrap().to_string()],
                distance);
            distances.insert(
                [cities.get(1).unwrap().to_string(),
                cities.get(0).unwrap().to_string()],
                distance);
        });
            
    Ok(distances)
}
