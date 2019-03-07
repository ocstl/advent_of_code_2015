use std::fs;

const PUZZLE_INPUT: u32 = 2503;

#[cfg(test)]
mod tests {
    use super::*;
}

#[derive(Hash, PartialEq, Eq)]
struct Reindeer {
    name: String,
    speed: u32,
    flight_time: u32,
    rest_time: u32,    
}

impl Reindeer {
    fn new(name: &str, speed: u32, flight_time: u32, rest_time: u32) ->
            Reindeer {
        Reindeer{name: name.to_string(), speed, flight_time, rest_time}
    }

    fn distance_traveled(&self, time: u32) -> u32 {
        let mut elapsed = 0;
        let mut distance = 0;
        
        while elapsed < time {
            distance += self.flight_time.min(time - elapsed) * self.speed;
            elapsed += self.flight_time + self.rest_time;
        }

        distance
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day14.txt";
    let reindeers = read_input(&filename)?;

    let first_answer = reindeers.iter()
        .map(|reindeer| reindeer.distance_traveled(PUZZLE_INPUT)).max().unwrap();
    println!("The first answer is: {}", first_answer);

    let second_answer = part2(&reindeers);
    println!("The second answer is: {}", second_answer);
    

    Ok(())
}

fn read_input(filename: &str) -> Result<Vec<Reindeer>, std::io::Error> {
    let result = fs::read_to_string(filename)?.lines()
        .map(move |line| {
            let mut iter = line.split_whitespace();
            let name = iter.nth(0).unwrap();
            let speed = iter.nth(2).unwrap().parse::<u32>().unwrap();
            let flight_time = iter.nth(2).unwrap().parse::<u32>().unwrap();
            let rest_time = iter.nth(6).unwrap().parse::<u32>().unwrap();
            Reindeer::new(name, speed, flight_time, rest_time)
        }).collect();

    Ok(result)
}

fn part2(reindeers: &[Reindeer]) -> u32 {
    let mut scores = vec![0;reindeers.len()];

    for t in 1..PUZZLE_INPUT+1 {
        let distances: Vec<u32> = reindeers.iter()
                .map(|reindeer| reindeer.distance_traveled(t))
                .collect();
                
        let max_distance = distances.iter().max().unwrap();
        
        distances.iter()
            .zip(scores.iter_mut())
            .for_each(|(d, s)| if d == max_distance {*s +=1;});
    }

    scores.into_iter().max().unwrap()

    
}
