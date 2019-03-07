use serde_json;
use serde_json::Value;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day12.txt";
    let json: Value =
        serde_json::from_slice(fs::read_to_string(&filename)?.as_bytes())
        .unwrap();
        
    let first_answer = sum(&json);
    println!("The first answer is: {}", first_answer);

    let second_answer = filtered_sum(&json);
    println!("The second answer is: {}", second_answer);
    
    Ok(())
}

fn sum(json: &Value) -> i64 {
    match json {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(j) => j.iter().map(|e| sum(e)).sum(),
        Value::Object(j) => j.values().map(|e| sum(e)).sum(),
        _ => 0,
    }
}

fn filtered_sum(json: &Value) -> i64 {
    match json {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(j) => j.iter().map(|e| filtered_sum(e)).sum(),
        Value::Object(j) => {
            if j.values().any(|e| e == "red") {
                return 0;
            }
            else {
                j.values().map(|e| filtered_sum(e)).sum()
            }
        },
        _ => 0,
    }
}
