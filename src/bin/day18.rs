use itertools::Itertools;
use std::fs;

struct LightGrid(Vec<Vec<bool>>);

impl LightGrid {
    fn from_input(filename: &str) -> Result<LightGrid, std::io::Error> {
        let contents = fs::read_to_string(filename)?;

        Ok(LightGrid(contents.lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect()))
    }

    fn nbr_lights_on(&self) -> usize {
        self.0.iter()
            .flat_map(|line| line.iter().filter(|&&b| b))
            .count()
    }

    fn step(&mut self) {
        let mut new_grid: Vec<Vec<bool>> = Vec::new();

        let max_y = self.0.len();
        let max_x = self.0[0].len();

        for y in 0..self.0.len() {
            let mut new_line: Vec<bool> = Vec::new();
            for x in 0..self.0[0].len() {
                let neighbors_on = (y.saturating_sub(1)..max_y.min(y+2))
                    .cartesian_product(x.saturating_sub(1)..max_x.min(x+2))
                    .filter(|&(y, x)| self.0[y][x])
                    .count();

                new_line.push(
                    if self.0[y][x] {
                        neighbors_on == 3 || neighbors_on == 4
                    }
                    else {
                        neighbors_on == 3
                    }
                );
            }

            new_grid.push(new_line);
        }

        self.0 = new_grid;
    }

    fn steps(&mut self, nbr_steps: usize) {
        for _ in 0..nbr_steps {
            self.step();
        }
    }

    fn sticky_lights(&mut self) {
        /* Make sure the four corners are on. */
        let dim_y = self.0.len() - 1;
        let dim_x = self.0[0].len() - 1;
        self.0[0][0] = true;
        self.0[0][dim_x] = true;
        self.0[dim_y][0] = true;
        self.0[dim_y][dim_x] = true;
    }

    fn step_part2(&mut self) {
        self.step();
        self.sticky_lights();
    }

    fn steps_part2(&mut self, nbr_steps: usize) {
        for _ in 0..nbr_steps {
            self.step_part2();
        }
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day18.txt";

    /* First part. */
    let mut lightgrid = LightGrid::from_input(&filename)?;
    lightgrid.steps(100);
    let first_answer = lightgrid.nbr_lights_on();
    println!("The first answer is: {}", first_answer);

    /* Second part. */
    let mut lightgrid = LightGrid::from_input(&filename)?;
    lightgrid.sticky_lights();
    lightgrid.steps_part2(100);
    let second_answer = lightgrid.nbr_lights_on();
    println!("The first answer is: {}", second_answer);
    
    Ok(())
}
