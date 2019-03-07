use std::fs;

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day20.txt";
    let puzzle_input: u32 = fs::read_to_string(&filename)?
        .trim()
        .parse()
        .unwrap();
        
    let first_answer = (1..).find(|&x| delivered_presents(x) >= puzzle_input).unwrap();
    println!("The first answer is: {}", first_answer);

    let second_answer = (1..)
        .find(|&x| delivered_presents_part2(x) >= puzzle_input).unwrap();
    println!("The second answer is: {}", second_answer);
    
    Ok(())
}

fn delivered_presents(house_number: u32) -> u32 {
    let end = (house_number as f64).sqrt() as u32 + 1;
    ((1..end).map(|x| {
        match (house_number % x, house_number / x) {
            (0, y) if y == x => x,
            (0, y) => x + y,
            _ => 0,
        }})
        .sum::<u32>()) * 10
}

fn delivered_presents_part2(house_number: u32) -> u32 {
    let end = (house_number as f64).sqrt() as u32 + 1;
    
    (1..end).map(|x| {
        match (house_number % x, house_number / x, x) {
            (0, a @ 1 ... 50, b @ 1 ... 50) =>
                if a == b { a } else { a + b },
            (0, 1 ... 50, b) => b,
            (0, a, 1 ... 50) => a,
            _ => 0,
        }})
        .sum::<u32>() * 11
}
