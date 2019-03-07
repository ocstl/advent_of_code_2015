use std::fs;

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = String::from("inputs/day1.txt");
    
    part1(&filename)?;
    part2(&filename)?;
    Ok(())
}

fn part1(filename: &str) -> Result<(), std::io::Error> {
    let result: isize = fs::read_to_string(filename)?
        .chars()
        .map(convert_parents)
        .sum();

    println!("The first answer is: {}", result);
    
    Ok(())
}

fn part2(filename: &str) -> Result<(), std::io::Error> {
    let result = fs::read_to_string(filename)?
        .chars()
        .map(convert_parents)
        .scan(0, |state, x| {*state = *state + x; Some(*state)})
        .position(|x| x == -1);

    println!("The second answer is: {}", result.unwrap_or(0) + 1);

    Ok(())  
}

fn convert_parents(c: char) -> isize {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}
