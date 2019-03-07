use std::collections::HashMap;
use std::fs;

enum Input {
    Signal(u16),
    Wire(String),
}

impl Input {
    fn new(s: &str) -> Input {
        match s.parse() {
            Ok(x) => Input::Signal(x),
            Err(_) => Input::Wire(s.to_string()),
        }
    }
}

enum Gate {
    Other(Input),
    NOT(Input),
    AND(Input, Input),
    OR(Input, Input),
    LSHIFT(Input, Input),
    RSHIFT(Input, Input),
}

impl Gate {
    fn new(s: &str) -> Gate {
        let v: Vec<&str> = s.split_whitespace().collect();

        match v.len() {
            1 => Gate::Other(Input::new(v.get(0).unwrap())),
            2 => Gate::NOT(Input::new(v.get(1).unwrap())),
            3 => {
                let x = Input::new(v.get(0).unwrap());
                let y = Input::new(v.get(2).unwrap());
                match v.get(1) {
                    Some(&"AND") => Gate::AND(x, y),
                    Some(&"OR") => Gate::OR(x, y),
                    Some(&"LSHIFT") => Gate::LSHIFT(x, y),
                    Some(&"RSHIFT") => Gate::RSHIFT(x, y),
                    _ => panic!("Instruction unreadable."),
                }
            },
            _ => panic!("Instruction unreadable."),
        }
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    let filename = "inputs/day7.txt";
    
    let mut circuit = build_circuit(&filename)?;
    let first_answer = get_signal_at(&mut circuit, &"a");
    println!("The first answer is: {}", first_answer);

    /* Second problem. Override b with the first answer (which insert allows us
     * to do). */
    circuit = build_circuit(&filename)?;
    circuit.insert("b".to_string(), Gate::Other(Input::Signal(first_answer)));
    let second_answer = get_signal_at(&mut circuit, &"a");
    
    println!("The second answer is: {}", second_answer);
    
    Ok(())
}

fn build_circuit(filename: &str) -> Result<HashMap<String, Gate>, std::io::Error> {
    let contents = fs::read_to_string(filename)?;

    let mut circuit = HashMap::new();

    contents.lines()
        .for_each(|line| {
            let mut iter = line.split(" -> ").to_owned();
            let gate = Gate::new(iter.next().unwrap());
            let name = iter.next().unwrap();

            circuit.insert(name.to_string(), gate);
        });

    Ok(circuit)
}

fn get_signal_at(mut circuit: &mut HashMap<String, Gate>, wire: &str) -> u16 {
    /* Have to remove to get around the immutable borrow. This may cause a panic
     * in case of a circular circuit, but this is actually a good thing, as a
     * circular circuit is not solvable. */
    let gate = circuit.remove(wire).unwrap();
    
    let signal = match gate {
        Gate::Other(w) => match w {
            Input::Signal(x) => x,
            Input::Wire(x) => get_signal_at(&mut circuit, &x),
        },
        Gate::NOT(w) => match w {
            Input::Signal(x) => !x,
            Input::Wire(x) => !get_signal_at(&mut circuit, &x),
        },
        Gate::AND(w1, w2) => {
            let x = match w1 {
                Input::Signal(x) => x,
                Input::Wire(x) => get_signal_at(&mut circuit, &x),
            };
            let y = match w2 {
                Input::Signal(x) => x,
                Input::Wire(x) => get_signal_at(&mut circuit, &x),
            };
            x & y
        },
        Gate::OR(w1, w2) => {
            let x = match w1 {
                Input::Signal(x) => x,
                Input::Wire(x) => get_signal_at(&mut circuit, &x),
            };
            let y = match w2 {
                Input::Signal(x) => x,
                Input::Wire(x) => get_signal_at(&mut circuit, &x),
            };
            x | y
        },
        Gate::LSHIFT(w1, w2) => {
            let x = match w1 {
                Input::Signal(x) => x,
                Input::Wire(x) => get_signal_at(&mut circuit, &x),
            };
            let y = match w2 {
                Input::Signal(x) => x,
                Input::Wire(x) => get_signal_at(&mut circuit, &x),
            };
            x << y
        },
        Gate::RSHIFT(w1, w2) => {
            let x = match w1 {
                Input::Signal(x) => x,
                Input::Wire(x) => get_signal_at(&mut circuit, &x),
            };
            let y = match w2 {
                Input::Signal(x) => x,
                Input::Wire(x) => get_signal_at(&mut circuit, &x),
            };
            x >> y
        },
    };

    /* Reinsert the removed entry. */
    circuit.insert(wire.to_string(), Gate::Other(Input::Signal(signal)));
    signal
}
