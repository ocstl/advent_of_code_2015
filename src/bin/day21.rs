use std::fs;

#[derive(PartialEq)]
struct Equipment {
    name: String,
    cost: usize,
    damage: usize,
    armor: usize,
}

impl Equipment {
    fn new(name: &str, cost: usize, damage: usize, armor: usize) -> Equipment {
        Equipment { name: name.to_string(), cost, damage, armor }
    }
}

#[derive(Debug)]
struct Unit {
    hp: usize,
    damage: usize,
    armor: usize,
}

impl Unit {
    fn new(hp: usize, damage: usize, armor: usize) -> Unit {
        Unit { hp, damage, armor }
    }

    fn simulate_battle(&self, other: &Unit) -> bool {
        let mut hp1 = self.hp;
        let mut hp2 = other.hp;

        let damage1 = 1.max(self.damage.saturating_sub(other.armor));
        let damage2 = 1.max(other.damage.saturating_sub(self.armor));
        
        loop {
            hp2 = hp2.saturating_sub(damage1);
            if hp2 == 0 {
                return true;
            }
            hp1 = hp1.saturating_sub(damage2);
            if hp1 == 0 {
                return false;
            }
        }
    }       
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day21.txt";
    let filename_store = "inputs/day21_2.txt";
    let boss = read_boss_file(&filename)?;
    let (weapons, armors, rings) = read_store_file(&filename_store)?;
    
    let units = generate_units(100, &weapons, &armors, &rings);

    let (wins, losses): (Vec<(usize, Unit)>, Vec<(usize, Unit)>) =
        units.into_iter().partition(|(_cost, unit)|
            unit.simulate_battle(&boss));
        
    let first_answer = wins.into_iter()
        .map(|(cost, _unit)| cost).min().unwrap_or(0);
    println!("The first answer is: {}", first_answer);

    let second_answer = losses.into_iter()
        .map(|(cost, _unit)| cost).max().unwrap_or(0);
    println!("The second answer is: {}", second_answer);
    
    Ok(())
}

fn read_boss_file(filename: &str) -> Result<Unit, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let mut iter = contents.lines();

    Ok(Unit::new(
        iter.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap(),
        iter.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap(),
        iter.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap(),
    ))
}

/* Requires removing the space in "Damage +1" et al. */
fn read_store_file(filename: &str) -> Result<(Vec<Equipment>, Vec<Equipment>,
                                              Vec<Equipment>), std::io::Error> {
    let mut weapons: Vec<Equipment> = Vec::new();
    let mut armors: Vec<Equipment> = Vec::new();
    let mut rings: Vec<Equipment> = Vec::new();
    let mut current = &mut weapons;

    let contents = fs::read_to_string(filename)?;

    for line in contents.lines() {
        if line == "" {
            /* Skip empty lines. */
        } else if line.starts_with("Weapons") {
            current = &mut weapons;
        } else if line.starts_with("Armor") {
            current = &mut armors;
        } else if line.starts_with("Rings") {
            current = &mut rings;
        } else {
            let mut iter = line.split_whitespace();
            current.push(Equipment::new(
                iter.next().unwrap(),
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            ));
        }
    }

    /* Add a no armor and no ring option. */
    armors.push(Equipment::new(&"None", 0, 0, 0));
    rings.push(Equipment::new(&"None", 0, 0, 0));    

    Ok((weapons, armors, rings))
}

fn generate_units(hp: usize, weapons: &[Equipment], armors: &[Equipment],
        rings: &[Equipment]) -> Vec<(usize, Unit)> {
    let mut units: Vec<(usize, Unit)> = Vec::new();

    for w in weapons {
        for a in armors {
            for r1 in rings {
                for r2 in rings {
                    if r1 != r2 || r1.name == "None" {
                        units.push((w.cost + a.cost + r1.cost + r2.cost,
                            Unit::new(
                                hp,
                                w.damage + a.damage + r1.damage + r2.damage,
                                w.armor + a.armor + r1.armor+ r2.armor
                            )));
                    }
                }
            }
        }
    }

    units
}
