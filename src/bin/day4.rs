extern crate md5;

use std::fs;

fn main() -> std::result::Result<(), std::io::Error> {
    let secret_key = fs::read_to_string("inputs/day4.txt")?;
    let secret_key = secret_key.trim();

    println!("The first answer is: {}", mine(secret_key, "00000").unwrap());
    println!("The second answer is: {}", mine(secret_key, "000000").unwrap());
    
    Ok(())
}

fn mine(secret_key: &str, difficulty: &str) -> Option<u32> {
    let result = (0..std::u32::MAX)
        .map(|c| (c, md5::compute(format!("{}{}", secret_key, c))))
        .find(|(_c, digest)| format!("{:x}", digest).starts_with(difficulty));

    match result {
        Some((x, _)) => Some(x),
        None => None,
    }
}
