use std::fs;

pub struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn from_input(s: &str) -> Present {
        let mut dims = s.split('x').map(|x| x.parse().unwrap());
        Present{length: dims.next().unwrap(),
                width: dims.next().unwrap(),
                height: dims.next().unwrap(),
        }
    }

    fn sides(&self) -> [u32; 3] {
        [self.length, self.width, self.height]
    }

    fn surface_area(&self) -> u32 {
        2 * (self.length * self.width
            + self.length * self.height
            + self.width * self.height)
    }

    fn smallest_side_area(&self) -> u32 {
        *[self.length * self.width,
        self.length * self.height,
        self.width * self.height]
            .iter().min().unwrap()
    }

    fn wrapping_paper(&self) -> u32 {
        self.surface_area() + self.smallest_side_area()
    }

    fn volume(&self) -> u32 {
        self.sides().iter().product()
    }

    fn ribbon(&self) -> u32 {
        let mut sides = self.sides();
        sides.sort();

        2 * (sides[0] + sides[1]) + self.volume()
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    let presents = read_input("inputs/day2.txt")?;
    
    part1(&presents);
    part2(&presents);
    
    Ok(())
}

fn part1(presents: &[Present]) {
    let result: u32 = presents.iter().map(|x| x.wrapping_paper()).sum();
    println!("The first answer is: {}", result);        
}

fn part2(presents: &[Present]) {
    let result: u32 = presents.iter().map(|x| x.ribbon()).sum();
    println!("The second answer is: {}", result);
}

fn read_input(filename: &str) -> Result<Vec<Present>, std::io::Error> {
    fs::read_to_string(filename)?
        .lines()
        .map(|line| Ok(Present::from_input(line)))
        .collect()
}
