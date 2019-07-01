use std::fs;

const FILENAME: &str = "inputs/day1.txt";

/// Santa is trying to deliver presents in a large apartment building, but he can't find the
/// right  floor - the directions he got are a little confusing. He starts on the ground floor
/// (floor 0) and then follows the instructions one character at a time.
///
/// An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ),
/// means he should go down one floor.
///
/// The apartment building is very tall, and the basement is very deep; he will never find the
/// top or bottom floors.
fn main() -> std::result::Result<(), std::io::Error> {
    let instructions = fs::read_to_string(FILENAME)?;

    part1(&instructions);
    part2(&instructions);
    Ok(())
}

/// To what floor do the instructions take Santa?
fn part1(instructions: &str) {
    let result: i32 = instructions.chars().filter_map(convert_parentheses).sum();

    println!("The first answer is: {}", result);
}

/// Now, given the same instructions, find the position of the first character that causes him to
/// enter the basement (floor -1). The first character in the instructions has position 1, the
/// second character has position 2, and so on.
fn part2(instructions: &str) {
    let result: usize = instructions
        .chars()
        .filter_map(convert_parentheses)
        .scan(0, |state, x| {
            *state = *state + x;
            Some(*state)
        })
        .position(|x| x == -1)
        .unwrap_or(0)
        // Convert to 1-indexing.
        + 1;

    println!("The second answer is: {}", result);
}

// Convert opening and closing parentheses to an up (+1) or down (-1) value, returning a None for
// other characters.
fn convert_parentheses(c: char) -> Option<i32> {
    match c {
        '(' => Some(1),
        ')' => Some(-1),
        _ => None,
    }
}
