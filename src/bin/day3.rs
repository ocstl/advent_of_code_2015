use std::collections::HashSet;
use std::fs;

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position{x, y}
    }

    fn update(&mut self, direction: char) {
        match direction {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '<' => self.x -= 1,
            '>' => self.x += 1,
            _ => (),
        }
    }

    fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = String::from("inputs/day3.txt");
    
    part1(&filename)?;
    part2(&filename)?;
    
    Ok(())
}

fn part1(filename: &str) -> Result<(), std::io::Error> {
    let mut visited_houses: HashSet<(i32, i32)> = HashSet::new();

    let mut position = Position::new(0, 0);
    visited_houses.insert(position.as_tuple());

    fs::read_to_string(filename)?
        .chars()
        .for_each(|c| {position.update(c); visited_houses.insert(position.as_tuple());});

    println!("The first answer is: {}", visited_houses.len());

    Ok(())
}

fn part2(filename: &str) -> Result<(), std::io::Error> {
    let mut visited_houses: HashSet<(i32, i32)> = HashSet::new();

    let mut santa_position = Position::new(0, 0);
    let mut robosanta_position = Position::new(0, 0);
    visited_houses.insert((0, 0));

    fs::read_to_string(filename)?
        .chars()
        .step_by(2)
        .for_each(|c| {santa_position.update(c); visited_houses.insert(santa_position.as_tuple());});

    fs::read_to_string(filename)?
        .chars()
        .skip(1)
        .step_by(2)
        .for_each(|c| {robosanta_position.update(c); visited_houses.insert(robosanta_position.as_tuple());});

    println!("The second answer is: {}", visited_houses.len());

    Ok(())
}
