use std::fs;
use std::usize;

/* We can store the spent mana in the variants. */
#[derive(Clone, Debug)]
enum Outcome {
    Win(usize),
    Loss(usize),
    Undecided(usize),
}

impl Outcome {
    fn value(&self) -> usize {
        match *self {
            Outcome::Win(x) => x,
            Outcome::Loss(x) => x,
            Outcome::Undecided(x) => x,
        }
    }

    fn win(&self, mana_spent: usize) -> Outcome {
        Outcome::Win(self.value() + mana_spent)
    }

    fn loss(&self, mana_spent: usize) -> Outcome {
        Outcome::Loss(self.value() + mana_spent)
    }

    fn undecided(&self, mana_spent: usize) -> Outcome {
        Outcome::Undecided(self.value() + mana_spent)
    }
}

#[derive(Clone, Debug)]
enum Difficulty {
    Normal,
    Hard,
}

#[derive(Clone, Debug)]
struct Unit {
    hp: usize,
    mana: usize,
    damage: usize,
}

impl Unit {
    fn new(hp: usize, mana: usize, damage: usize) -> Unit {
        Unit { hp, mana, damage }
    }
}

#[derive(Clone, Debug)]
struct Effects {
    shield: usize,
    poison: usize,
    recharge: usize,
}

impl Effects {
    fn new() -> Effects {
        Effects { shield: 0, poison: 0, recharge: 0 }
    }
}

#[derive(Clone, Debug)]
struct Battle {
    player: Unit,
    boss: Unit,
    difficulty: Difficulty,
    state: Outcome,
    effects: Effects,
}

impl Battle {
    fn new(player: &Unit, boss: &Unit, difficulty: Difficulty) -> Battle {
        Battle {
            player: player.clone(),
            boss: boss.clone(),
            difficulty: difficulty,
            state: Outcome::Undecided(0),
            effects: Effects::new()
        }
    }

    fn apply_effects(&mut self) {            
        self.effects.shield = self.effects.shield.saturating_sub(1);

        if self.effects.poison > 0 {
            self.boss.hp = self.boss.hp.saturating_sub(3);
            self.effects.poison -= 1;
        }

        if self.effects.recharge > 0 {
            self.player.mana += 101;
            self.effects.recharge -= 1;
        }
    }

    fn boss_turn(&mut self) {
        let damage = match self.effects.shield {
            0 => self.boss.damage,
            _ => 1.min(self.boss.damage.saturating_sub(7)),
        };
        
        self.apply_effects();

        if self.player.hp == 0 {
            self.state = self.state.loss(0);
        } else if self.boss.hp == 0 {
            self.state = self.state.win(0);
        } else {
            self.player.hp = self.player.hp.saturating_sub(damage);
            if self.player.hp == 0 {
                self.state = self.state.loss(0);
            }
        }
    }

    fn magic_missile(&mut self) -> Option<()> {
        let mana_cost = 53;

        match self.difficulty {
            Difficulty::Normal => (),
            Difficulty::Hard =>
                self.player.hp = self.player.hp.saturating_sub(1),
        }        
        self.apply_effects();

        if self.player.hp == 0 {
            self.state = self.state.loss(0);
            return Some(());
        } else if self.boss.hp == 0 {
            self.state = self.state.win(0);
            return Some(());
        }

        /* If the player doesn't have enough mana, return a None. */
        match self.player.mana.checked_sub(mana_cost) {
            None => return None,
            Some(x) => {
                self.player.mana = x;
                self.boss.hp = self.boss.hp.saturating_sub(4);
                self.state = self.state.undecided(mana_cost);
            },
        }
        
        self.boss_turn();
        Some(())
    }

    fn drain(&mut self) -> Option<()> {
        let mana_cost = 73;
        
        match self.difficulty {
            Difficulty::Normal => (),
            Difficulty::Hard =>
                self.player.hp = self.player.hp.saturating_sub(1),
        }        
        self.apply_effects();

        if self.player.hp == 0 {
            self.state = self.state.loss(0);
            return Some(());
        } else if self.boss.hp == 0 {
            self.state = self.state.win(0);
            return Some(());
        }

        /* If the player doesn't have enough mana, return a None. */
        match self.player.mana.checked_sub(mana_cost) {
            None => return None,
            Some(x) => {
                self.player.mana = x;
                self.boss.hp = self.boss.hp.saturating_sub(2);
                self.player.hp += 2;
                self.state = self.state.undecided(mana_cost);
            },
        }
        
        self.boss_turn();
        Some(())
    }

    fn shield(&mut self) -> Option<()> {
        let mana_cost = 113;
        
        match self.difficulty {
            Difficulty::Normal => (),
            Difficulty::Hard =>
                self.player.hp = self.player.hp.saturating_sub(1),
        }        
        self.apply_effects();

        if self.player.hp == 0 {
            self.state = self.state.loss(0);
            return Some(());
        } else if self.boss.hp == 0 {
            self.state = self.state.win(0);
            return Some(());
        }

        /* If the player doesn't have enough mana or there is already a shield
         * effect, return a None. */
        if self.effects.shield > 0 {
            return None;
        } else {
            match self.player.mana.checked_sub(mana_cost) {
                None => return None,
                Some(x) => {
                    self.player.mana = x;
                    self.effects.shield = 6;
                    self.state = self.state.undecided(mana_cost);
                },
            }
        }
        
        self.boss_turn();
        Some(())
    }

    fn poison(&mut self) -> Option<()> {
        let mana_cost = 173;

        match self.difficulty {
            Difficulty::Normal => (),
            Difficulty::Hard =>
                self.player.hp = self.player.hp.saturating_sub(1),
        }        
        self.apply_effects();

        if self.player.hp == 0 {
            self.state = self.state.loss(0);
            return Some(());
        } else if self.boss.hp == 0 {
            self.state = self.state.win(0);
            return Some(());
        }

        /* If the player doesn't have enough mana or there is already a poison
         * effect, return a None. */
        if self.effects.poison > 0 {
            return None;
        } else {
            match self.player.mana.checked_sub(mana_cost) {
                None => return None,
                Some(x) => {
                    self.player.mana = x;
                    self.effects.poison = 6;
                    self.state = self.state.undecided(mana_cost);
                },
            }
        }
        
        self.boss_turn();
        Some(())
    }

    fn recharge(&mut self) -> Option<()> {
        let mana_cost = 229;

        match self.difficulty {
            Difficulty::Normal => (),
            Difficulty::Hard =>
                self.player.hp = self.player.hp.saturating_sub(1),
        }        
        self.apply_effects();

        if self.player.hp == 0 {
            self.state = self.state.loss(0);
            return Some(());
        } else if self.boss.hp == 0 {
            self.state = self.state.win(0);
            return Some(());
        }

        /* If the player doesn't have enough mana or there is already a recharge
         * effect, return a None. */
        if self.effects.recharge > 0 {
            return None;
        } else {
            match self.player.mana.checked_sub(mana_cost) {
                None => return None,
                Some(x) => {
                    self.player.mana = x;
                    self.effects.recharge = 5;
                    self.state = self.state.undecided(mana_cost);
                },
            }
        }
        
        self.boss_turn();
        Some(())
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day22.txt";
    let boss = read_boss_file(&filename)?;
    let player = Unit::new(50, 500, 0);

    let initial_state = Battle::new(&player, &boss, Difficulty::Normal);
    let first_answer = part1(&initial_state);
    println!("The first answer is: {}", first_answer);

    let hard_battle = Battle::new(&player, &boss, Difficulty::Hard);
    let second_answer = part1(&hard_battle);
    println!("The second answer is: {}", second_answer);
    
    Ok(())
}

fn read_boss_file(filename: &str) -> Result<Unit, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let mut iter = contents.lines();

    Ok(Unit::new(
        iter.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap(),
        0,
        iter.next().unwrap().split(": ").nth(1).unwrap().parse().unwrap(),
    ))
}

fn part1(battle: &Battle) -> usize {
    let mut current_min = usize::MAX;
    let mut to_do: Vec<Battle> = Vec::new();

    to_do.push(battle.clone());
    while let Some(current) = to_do.pop() {
        if current.state.value() > current_min {
            continue
        }

        let mut magic_missile = current.clone();
        if let Some(_) = magic_missile.magic_missile() {
            match magic_missile.state {
                Outcome::Win(x) => current_min = current_min.min(x),
                Outcome::Undecided(x) if x < current_min =>
                    to_do.push(magic_missile),
                _ => (),
            }
        }

        let mut drain = current.clone();
        if let Some(_) = drain.drain() {
            match drain.state {
                Outcome::Win(x) => current_min = current_min.min(x),
                Outcome::Undecided(x) if x < current_min =>
                    to_do.push(drain),
                _ => (),
            }
        }

        let mut shield = current.clone();
        if let Some(_) = shield.shield() {
            match shield.state {
                Outcome::Win(x) => current_min = current_min.min(x),
                Outcome::Undecided(x) if x < current_min =>
                    to_do.push(shield),
                _ => (),
            }
        }

        let mut poison = current.clone();
        if let Some(_) = poison.poison() {
            match poison.state {
                Outcome::Win(x) => current_min = current_min.min(x),
                Outcome::Undecided(x) if x < current_min =>
                    to_do.push(poison),
                _ => (),
            }
        }

        let mut recharge = current.clone();
        if let Some(_) = recharge.recharge() {
            match recharge.state {
                Outcome::Win(x) => current_min = current_min.min(x),
                Outcome::Undecided(x) if x < current_min =>
                    to_do.push(recharge),
                _ => (),
            }
        }
    }

    current_min
}
