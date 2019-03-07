use itertools::Itertools;
use std::fs;

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day24.txt";
    let contents = fs::read_to_string(&filename)?;
    let weights: Vec<usize> =
        contents.lines().map(|line| line.parse::<usize>().unwrap()).collect();

    let first_answer = part1(&weights, 3);
    println!("The first answer is: {}", first_answer);

    let second_answer = part1(&weights, 4);
    println!("The second answer is: {}", second_answer);
    
    Ok(())
}

/* We are assuming that the remaining groups can be split evenly. A recursive
 * call with the remaining packages (using an Option<usize> instead) would be
 * the better solution. */
fn part1(weights: &[usize], nbr_groups: usize) -> usize {
    let target_weight: usize = weights.iter().sum::<usize>() / nbr_groups;
    let mut size = 1;
    
    loop {
        let result = weights.into_iter()
                        .combinations(size)
                        .filter_map(|c|
                            if c.clone().into_iter().sum::<usize>() == target_weight {
                                Some(c.into_iter().product())
                            } else {
                                None
                            })
                        .min();

        match result {
            Some(x) => return x,
            None => size += 1,
        }
    }
}
