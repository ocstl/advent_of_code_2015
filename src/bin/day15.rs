use std::fs;

const PUZZLE_INPUT: i32 = 100;
const MAX_CALORIES: i32 = 500;

#[cfg(test)]
mod tests {
    use super::*;
}

#[derive(Clone, Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn from_str(s: &str) -> Ingredient {
        let s = s.chars().filter(|&c| c != ',' && c != ':').collect::<String>();
        let mut iter = s.split_whitespace();
        Ingredient {
            name: iter.nth(0).unwrap().to_string(),
            capacity: iter.nth(1).unwrap().parse().unwrap(),
            durability: iter.nth(1).unwrap().parse().unwrap(),
            flavor: iter.nth(1).unwrap().parse().unwrap(),
            texture: iter.nth(1).unwrap().parse().unwrap(),
            calories: iter.nth(1).unwrap().parse().unwrap(),
        }
    }
}

#[derive(Clone)]
struct CookieRecipe {
    ingredients: Vec<(Ingredient, i32)>,
    teaspoons: i32,
}

impl CookieRecipe {
    fn from_ingredients(ingredients: &[Ingredient], teaspoons: i32)
            -> CookieRecipe {
        CookieRecipe {
            ingredients: ingredients.iter().map(|i| (i.clone(), 0)).collect(),
            teaspoons: teaspoons,
        }
    }

    fn capacity(&self) -> i32 {
        self.ingredients.iter()
            .map(|(i, qty)| qty * i.capacity)
            .sum::<i32>()
            .max(0)
    }

    fn durability(&self) -> i32 {
        self.ingredients.iter()
            .map(|(i, qty)| qty * i.durability)
            .sum::<i32>()
            .max(0)
    }

    fn flavor(&self) -> i32 {
        self.ingredients.iter()
            .map(|(i, qty)| qty * i.flavor)
            .sum::<i32>()
            .max(0)
    }

    fn texture(&self) -> i32 {
        self.ingredients.iter()
            .map(|(i, qty)| qty * i.texture)
            .sum::<i32>()
            .max(0)
    }

    fn calories(&self) -> i32 {
        self.ingredients.iter()
            .map(|(i, qty)| qty * i.calories)
            .sum::<i32>()
    }

    fn score(&self) -> i32 {
        self.capacity() * self.durability() * self.flavor() * self.texture()
    }

    fn increase_quantity(&mut self) -> Option<()> {
        let mut iter = self.ingredients.iter_mut();
        loop {
            match iter.next() {
                Some((_, q)) if *q == self.teaspoons => *q = 0,
                Some((_, q)) => {*q += 1; return Some(());},
                None => return None,
            }
        }
    }
}

impl Iterator for CookieRecipe {
    type Item = CookieRecipe;

    fn next(&mut self) -> Option<Self::Item> {
        if self.increase_quantity() == None {
            return None;
        }
        
        while self.ingredients.iter().map(|(_, q)| q).sum::<i32>() != self.teaspoons {
            match self.increase_quantity() {
                Some(_) => (),
                None => return None,
            }
        }

        Some(self.clone())
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day15.txt";

    let cookie_recipe = read_input(&filename)?;

    let first_answer = cookie_recipe.map(|c| c.score()).max().unwrap();
    println!("The first answer is: {}", first_answer);

    let cookie_recipe = read_input(&filename)?;
    let second_answer = cookie_recipe
        .filter_map(|c|
            if c.calories() <= MAX_CALORIES {Some(c.score())} else {None})
        .max()
        .unwrap();
    println!("The second answer is: {}", second_answer);
            

    Ok(())
}

fn read_input(filename: &str) -> Result<CookieRecipe, std::io::Error> {
    let ingredients: Vec<Ingredient> = fs::read_to_string(filename)?.lines()
        .map(|line| Ingredient::from_str(line))
        .collect();

    Ok(CookieRecipe::from_ingredients(&ingredients, PUZZLE_INPUT))
}
