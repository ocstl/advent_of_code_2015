use std::collections::HashSet;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_password() {
        let actual = AocPassword::from_slice(b"aa").next();
        let expected = [b'a', b'b'];
        assert_eq!(actual.unwrap(), expected)
    }

    #[test]
    fn test_generator() {
        let actual = AocPassword::from_slice(b"abcdefgh").new_password();
        let expected = String::from("abcdffaa");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_generator_2() {
        let actual = AocPassword::from_slice(b"ghijklmn").new_password();
        let expected = String::from("ghjaabcc");
        assert_eq!(actual, expected);
    }    
}

struct AocPassword(Vec<u8>);

impl AocPassword {
    fn from_slice(v: &[u8]) -> AocPassword {
        AocPassword(v.to_vec())
    }

    fn increasing_letters(&self) -> bool {
        self.0.windows(3)
            .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
    }

    fn no_confusing_letter(&self) -> bool {
        !self.0.iter().any(|c| b"iol".contains(c))
    }

    fn two_different_pairs(&self) -> bool {
        let mut h = HashSet::new();

        self.0.windows(2)
            .for_each(|w| if w[0] == w[1] {h.insert(w[0]);});

        h.len() > 1
    }

    fn new_password(&mut self) -> String {
        self.next();
        while !self.increasing_letters() || !self.no_confusing_letter() ||
                !self.two_different_pairs() {
            self.next();
        }

        String::from_utf8_lossy(&self.0).to_string()
    }
}

impl Iterator for AocPassword {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut increment = true;
        let mut iter = self.0.iter_mut().rev();

        while increment {
            match iter.next() {
                Some(x) =>
                    *x = match x {
                        b'z' => b'a',
                        _ => {increment = false; *x + 1},
                    },
                None => (),
            }
        }

        if increment {
            self.0.insert(0, b'a');
        }
        
        Some(self.0.clone())
    }       
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day11.txt";
    let mut password = AocPassword::from_slice(
        fs::read_to_string(&filename)?.trim().as_bytes()
    );

    let first_answer = password.new_password();
    println!("The first answer is: {}", first_answer);

    let second_answer = password.new_password();
    println!("The second answer is: {}", second_answer);
    
    Ok(())
}
