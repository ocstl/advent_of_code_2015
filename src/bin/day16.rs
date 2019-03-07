use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Aunt {
    name: String,
    sample: HashMap<String, u32>,
}

impl Aunt {
    fn new() -> Aunt {
        Aunt { name: String::new(), sample: HashMap::new() }
    }

    fn add_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn add_compound(&mut self, compound: &str, amount: u32) {
        match compound {
            "children" | "cats" | "samoyeds" | "pomeranians" | "akitas" |
            "vizslas" | "goldfish" | "trees" | "cars" |"perfumes"
                => {self.sample.insert(compound.to_string(), amount);},
            _ => panic!("Compound not allowed."),
        }
    }

    fn compare<F>(&self, comparison_function: F) -> bool
        where F: Fn(&str) -> u32 {
            
        self.sample.iter()
            .all(|(k, v)| *v == comparison_function(k))
    }

    /* Update for the outdated retroencabulator. */
    fn compare2<F>(&self, comparison_function: F) -> bool
        where F: Fn(&str) -> u32 {
            
        self.sample.iter()
            .all(|(k, v)| match k.as_str() {
                "cats" | "trees" => *v > comparison_function(k),
                "pomeranians" | "goldfish" => *v < comparison_function(k),
                _ => *v == comparison_function(k),
            })
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day16.txt";
    let aunts = read_input(&filename)?;

    let first_answer = aunts.iter().find(|aunt| aunt.compare(sample)).unwrap();
    println!("The first answer is: {}", first_answer.name);

    let second_answer = aunts.iter().find(|aunt| aunt.compare2(sample)).unwrap();
    println!("The second answer is: {}", second_answer.name);

    Ok(())
}

fn read_input(filename: &str) -> Result<Vec<Aunt>, std::io::Error> {
    let mut aunts = Vec::new();
    
    let contents = fs::read_to_string(filename)?;
    for line in contents.lines() {
        let mut aunt = Aunt::new();

        let mut iter = line.splitn(2, ": ");
        aunt.add_name(iter.next().unwrap());

        let mut it = iter.next().unwrap().split(", ");
        while let Some(compound) = it.next() {
            let mut name_amount = compound.split(": ");
            aunt.add_compound(name_amount.next().unwrap(),
                name_amount.next().unwrap().parse().unwrap());
        }
        aunts.push(aunt);
    }        

    Ok(aunts)
}

/* Can't build a const HashMap (yet!), so one can use a function to check
 * against the sample given in the problem definition.
 */
fn sample(compound: &str) -> u32 {
    match compound {
        "children" => 3,
        "cats" => 7,
        "samoyeds" => 2,
        "pomeranians" => 3,
        "akitas" => 0,
        "vizslas" => 0,
        "goldfish" => 5,
        "trees" => 3,
        "cars" => 2,
        "perfumes" => 1,
        _ => panic!("Unknown compound."),
    }
}
