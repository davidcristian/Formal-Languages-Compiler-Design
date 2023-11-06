// TODO: use hash map from 3/
use std::collections::HashMap;

pub struct Automaton {
    alphabet: Vec<char>,
    initial_state: usize,
    final_states: Vec<usize>,
    transitions: HashMap<(usize, char), usize>,
}

impl Automaton {
    pub fn new(file_path: &str) -> Result<Self, String> {
        let mut automaton = Self {
            alphabet: Vec::new(),
            initial_state: 0,
            final_states: Vec::new(),
            transitions: HashMap::new(),
        };

        match automaton.parse_file(file_path) {
            Ok(_) => Ok(automaton),
            Err(e) => Err(e),
        }
    }

    fn parse_file(&mut self, file_path: &str) -> Result<(), String> {
        let file = match std::fs::read_to_string(file_path) {
            Ok(file) => file,
            Err(e) => {
                let error = format!("could not read finite automaton file: {}", e.to_string());
                return Err(error);
            }
        };
        let mut lines = file.lines();

        // read alphabet
        let alphabet = match lines.next() {
            Some(alphabet) => alphabet,
            None => {
                let error = format!("invalid finite automaton file: missing alphabet");
                return Err(error);
            }
        };
        self.alphabet = alphabet.chars().collect();

        // read initial state
        let initial_state = match lines.next() {
            Some(initial_state) => initial_state,
            None => {
                let error = format!("invalid finite automaton file: missing initial state");
                return Err(error);
            }
        };
        self.initial_state = match initial_state.parse::<usize>() {
            Ok(initial_state) => initial_state,
            Err(e) => {
                let error = format!("invalid initial state: {}", e.to_string());
                return Err(error);
            }
        };

        // read final states
        let final_states = match lines.next() {
            Some(final_states) => final_states,
            None => {
                let error = format!("invalid finite automaton file: missing final states");
                return Err(error);
            }
        };
        for final_state in final_states.split_whitespace() {
            match final_state.parse::<usize>() {
                Ok(final_state) => self.final_states.push(final_state),
                Err(e) => {
                    let error = format!("invalid final state '{}': {}", final_state, e.to_string());
                    return Err(error);
                }
            }
        }

        // read transitions
        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 3 {
                let err = format!("invalid transition: {}", line);
                return Err(err);
            }

            let start_state = match parts[0].parse::<usize>() {
                Ok(start_state) => start_state,
                Err(e) => {
                    let error = format!("invalid start state '{}': {}", parts[0], e.to_string());
                    return Err(error);
                }
            };

            if parts[1].len() != 1 {
                let error = format!("invalid symbol '{}' for transition '{}'", parts[1], line);
                return Err(error);
            }
            let symbol = match parts[1].chars().next() {
                Some(symbol) => symbol,
                None => {
                    let error = format!("missing symbol for transition '{}'", line);
                    return Err(error);
                }
            };
            if !self.alphabet.contains(&symbol) {
                let error = format!("missing character '{}' for transition '{}'", symbol, line);
                return Err(error);
            }

            let end_state = match parts[2].parse::<usize>() {
                Ok(end_state) => end_state,
                Err(e) => {
                    let error = format!("invalid end state '{}': {}", parts[2], e.to_string());
                    return Err(error);
                }
            };

            self.transitions.insert((start_state, symbol), end_state);
        }

        Ok(())
    }

    pub fn validate(&self, sequence: &str) -> bool {
        let mut current_state = self.initial_state;
        for symbol in sequence.chars() {
            if let Some(&next_state) = self.transitions.get(&(current_state, symbol)) {
                current_state = next_state;
            } else {
                return false;
            }
        }

        self.final_states.contains(&current_state)
    }

    #[allow(dead_code)]
    pub fn display(&self) {
        println!("Alphabet: {:?}", self.alphabet);
        println!("Initial state: {}", self.initial_state);
        println!("Final states: {:?}", self.final_states);

        println!("Transitions:");
        for ((start_state, symbol), end_state) in &self.transitions {
            println!("{} {} {}", start_state, symbol, end_state);
        }
    }
}
