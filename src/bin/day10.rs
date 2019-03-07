use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn look_and_say_1() {
        assert_eq!(look_and_say(&"1\n"), String::from("11\n"));
    }

    #[test]
    fn look_and_say_11() {
        assert_eq!(look_and_say(&"11\n"), String::from("21\n"));
    }

    #[test]
    fn look_and_say_21() {
        assert_eq!(look_and_say(&"21\n"), String::from("1211\n"));
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day10.txt";
    let puzzle_input = fs::read_to_string(filename)?;

    /* Remember to remove the line feed when counting length. */
    let first_answer = (0..40)
        .fold(puzzle_input, |outcome, _| look_and_say(&outcome));
    println!("The first answer is: {}", first_answer.len() - 1);

    let second_answer = (40..50)
        .fold(first_answer, |outcome, _| look_and_say(&outcome));
    println!("The first answer is: {}", second_answer.len() - 1);
    
    Ok(())
}

fn look_and_say(s: &str) -> String {
    let mut result = String::new();
    let mut capture = (s.chars().next().unwrap(), 0);

    s.chars()
        .for_each(|c|
            if capture.0 == c {
                capture.1 += 1;
            }
            else {
                result.push_str(&capture.1.to_string());
                result.push(capture.0);
                capture = (c, 1);
            }
        );
        
    result.push('\n');
    result
}
