use std::fs;
use std::u8;

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day8.txt";

    let first_answer = part1(&filename)?;
    println!("The first answer is: {}", first_answer);

    let second_answer = part2(&filename)?;
    println!("The second answer is: {}", second_answer);
    
    Ok(())
}

fn part1(filename: &str) -> Result<usize, std::io::Error> {
    let contents = fs::read_to_string(filename)?;

    /* Need to add 2 for the double quotes. */
    Ok(contents.lines()
        .fold(0, |acc, line| acc + line.len() - decode(line).len()))
}

fn part2(filename: &str) -> Result<usize, std::io::Error> {
    let contents = fs::read_to_string(filename)?;

    /* Add 1 for each special character (\, "), plus 2 for the extra quotes. */
    Ok(contents.lines()
        .fold(0, |acc, line| acc + 2 + line.matches(r"\").count()
                                 + line.matches(r#"""#).count()))
}

fn decode(s: &str) -> Vec<u8> {
    let s = s.as_bytes();

    let mut result = Vec::new();
    /* Ignore the first double quotes. */
    let mut idx = 1;

    while idx < s.len() {
        match s[idx] {
            b'\\' => {
                idx += 1;
                match s[idx] {
                    b'x' => {
                        result.push(u8::from_str_radix(&String::from_utf8(s[idx+1..idx+3].to_vec()).unwrap(), 16).unwrap());
                        idx += 3;
                    }
                    _ => {
                        result.push(s[idx]);
                        idx += 1;
                    }
                }
            },
            _ => {
                result.push(s[idx]);
                idx += 1;
            },
        }
    }

    /* Remove the last double quotes. */
    result.pop();
    result
}
