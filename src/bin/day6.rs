use std::fs;

struct LightGrid ([[bool; 1000]; 1000]);

impl LightGrid {
    fn new() -> LightGrid {
        LightGrid([[false; 1000]; 1000])
    }

    fn turn_on(&mut self, start: &(usize, usize), end: &(usize, usize)) {
        (start.0..end.0+1)
            .for_each(|x|
                (start.1..end.1+1).for_each(|y| self.0[x][y] = true)
            );
    }

    fn turn_off(&mut self, start: &(usize, usize), end: &(usize, usize)) {
        (start.0..end.0+1)
            .for_each(|x|
                (start.1..end.1+1).for_each(|y| self.0[x][y] = false)
            );
    }

    fn toggle(&mut self, start: &(usize, usize), end: &(usize, usize)) {
        (start.0..end.0+1)
            .for_each(|x|
                (start.1..end.1+1).for_each(|y| self.0[x][y] ^= true)
            );
    }

    fn nbr_lights_on(&self) -> u32 {
        self.0.iter().
            map(|line| line.iter().map(|&x| x as u32).sum::<u32>())
            .sum()
    }   
}

struct LightgridPart2 ([[u32; 1000]; 1000]);

impl LightgridPart2 {
    fn new() -> LightgridPart2 {
        LightgridPart2([[0; 1000]; 1000])
    }

    fn turn_on(&mut self, start: &(usize, usize), end: &(usize, usize)) {
        (start.0..end.0+1)
            .for_each(|x|
                (start.1..end.1+1).for_each(|y| self.0[x][y] += 1)
            );
    }

    fn turn_off(&mut self, start: &(usize, usize), end: &(usize, usize)) {
        (start.0..end.0+1)
            .for_each(|x|
                (start.1..end.1+1).for_each(
                    |y| self.0[x][y] = self.0[x][y].saturating_sub(1)
                )
            );
    }

    fn toggle(&mut self, start: &(usize, usize), end: &(usize, usize)) {
        (start.0..end.0+1)
            .for_each(|x|
                (start.1..end.1+1).for_each(|y| self.0[x][y] += 2)
            );
    }

    fn nbr_lights_on(&self) -> u32 {
        self.0.iter().map(|line| line.iter()).flatten().sum()
    }   
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day6.txt";

    part1(&filename)?;
    part2(&filename)?;
    
    Ok(())
}

fn part1(filename: &str) -> Result<(), std::io::Error> {
    let mut light_grid = LightGrid::new();
    
    let contents = fs::read_to_string(filename)?;

    contents.lines()
        .for_each(|line| {
                /* Keep in mind we are doing a reverse split, to get a full
                 * instruction ("toggle", "turn on", "turn off"). */
                let instruction: Vec<&str> = line.rsplitn(4, ' ').collect();
                let start_idx: Vec<usize> = instruction.get(2).unwrap().split(',').map(|s| s.parse::<usize>().expect("Not a number.")).collect();
                let start_idx = (start_idx.get(0).unwrap().clone(), start_idx.get(1).unwrap().clone());
                let end_idx: Vec<usize> = instruction.get(0).unwrap().split(',').map(|s| s.parse::<usize>().expect("Not a number.")).collect();
                let end_idx = (end_idx.get(0).unwrap().clone(), end_idx.get(1).unwrap().clone());

                match instruction.last() {
                    Some(&"turn off") => light_grid.turn_off(&start_idx, &end_idx),
                    Some(&"turn on") => light_grid.turn_on(&start_idx, &end_idx),
                    Some(&"toggle") => light_grid.toggle(&start_idx, &end_idx),
                    _ => (),
                }
            });

    println!("The first answer is: {}", light_grid.nbr_lights_on());

    Ok(())
}

fn part2(filename: &str) -> Result<(), std::io::Error> {
    let mut light_grid = LightgridPart2::new();
    
    let contents = fs::read_to_string(filename)?;

    contents.lines()
        .for_each(|line| {
                /* Keep in mind we are doing a reverse split, to get a full
                 * instruction ("toggle", "turn on", "turn off"). */
                let instruction: Vec<&str> = line.rsplitn(4, ' ').collect();
                let start_idx: Vec<usize> = instruction.get(2).unwrap().split(',').map(|s| s.parse::<usize>().expect("Not a number.")).collect();
                let start_idx = (start_idx.get(0).unwrap().clone(), start_idx.get(1).unwrap().clone());
                let end_idx: Vec<usize> = instruction.get(0).unwrap().split(',').map(|s| s.parse::<usize>().expect("Not a number.")).collect();
                let end_idx = (end_idx.get(0).unwrap().clone(), end_idx.get(1).unwrap().clone());

                match instruction.last() {
                    Some(&"turn off") => light_grid.turn_off(&start_idx, &end_idx),
                    Some(&"turn on") => light_grid.turn_on(&start_idx, &end_idx),
                    Some(&"toggle") => light_grid.toggle(&start_idx, &end_idx),
                    _ => (),
                }
            });

    println!("The second answer is: {}", light_grid.nbr_lights_on());

    Ok(())
}
