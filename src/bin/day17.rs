use std::fs;

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day17.txt";
    let containers = read_input(&filename)?;

    let first_answer = count_combinations(&containers, 150);
    println!("The first answer is: {}", first_answer);

    let second_answer = count_combinations_part2(&containers, 150);
    println!("The second answer is: {}", second_answer);

    Ok(())
}

fn read_input(filename: &str) -> Result<Vec<usize>, std::io::Error> {
    let result = fs::read_to_string(filename)?.lines()
        .map(|line| line.parse().unwrap())
        .collect();

    Ok(result)
}

fn count_combinations(containers: &[usize], amount: usize) -> usize {
    let max_nbr = 1_usize << containers.len();

    /* Use a yes/no vector (usize). */
    (0..max_nbr).filter(|x| 
        containers.iter()
            .enumerate()
            .map(|(idx, c)| c * ((x >> idx) & 1))
            .sum::<usize>() == amount)
        .count()
}

/* Find the number of combinations using the smallest number of containers. */
fn count_combinations_part2(containers: &[usize], amount: usize) -> usize {
    let max_nbr = 1_usize << containers.len();

    /* Use a yes/no vector (usize). */
    let nbr_containers: Vec<usize> = (0..max_nbr).filter_map(|x| 
        if containers.iter()
                    .enumerate()
                    .map(|(idx, c)| c * ((x >> idx) & 1))
                    .sum::<usize>() == amount {
            Some(x.count_ones() as usize)
        }
        else {
            None
        })
        .collect();

    let min_nbr_containers = nbr_containers.iter().min().unwrap();
    nbr_containers.iter().filter(|&n| n == min_nbr_containers).count()
}
