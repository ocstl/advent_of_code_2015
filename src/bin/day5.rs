use std::fs;

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day5.txt";
    
    part1(&filename)?;
    part2(&filename)?;
    
    Ok(())
}

fn part1(filename: &str) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(filename)?;

    let result = contents.lines()
        .filter(|line| !contains_bad_strings(line)
            && double_letter(line)
            && (vowel_count(line) >= 3)
        )
        .count();

    println!("The first answer is: {}", result);

    Ok(())
}

fn vowel_count(s: &str) -> usize {
    s.chars()
        .filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
        .count()
}

fn double_letter(s: &str) -> bool {
    s.as_bytes().windows(2).any(|window| window[0] == window[1])
}

fn contains_bad_strings(s: &str) -> bool {
    s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
}

fn part2(filename: &str) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(filename)?;

    let result = contents.lines()
        .filter(|line| repeating_pair(&line) && repeating_letter(&line))
        .count();

    println!("The second answer is: {}", result);

    Ok(())
}

fn repeating_pair(s: &str) -> bool {
    (0..s.len()-1).any(|idx| s[idx+2..].contains(&s[idx..idx+2]))
}

fn repeating_letter(s: &str) -> bool {
    s[0..s.len()-2].char_indices()
        .any(|(idx, c)| c.to_string() == s[idx+2..idx+3])
}
