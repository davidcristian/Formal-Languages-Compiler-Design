use hash_map::HashMap;
use std::collections::HashSet as Set;

pub type State = usize;

pub struct Automaton {
    states: Set<State>,

    alphabet: Set<char>,
    initial_state: State,
    final_states: Set<State>,
    transitions: HashMap<(State, char), State>,
}

impl Automaton {
    pub fn new(file_path: &str) -> Result<Self, String> {
        let mut automaton = Self {
            states: Set::new(),

            alphabet: Set::new(),
            initial_state: 0,
            final_states: Set::new(),
            transitions: HashMap::new(),
        };

        match automaton.parse_file(file_path) {
            Ok(_) => Ok(automaton),
            Err(e) => Err(e),
        }
    }

    pub fn get_states(&self) -> &Set<State> {
        &self.states
    }

    pub fn get_alphabet(&self) -> &Set<char> {
        &self.alphabet
    }

    pub fn get_initial_state(&self) -> &State {
        &self.initial_state
    }

    pub fn get_final_states(&self) -> &Set<State> {
        &self.final_states
    }

    pub fn get_transitions(&self) -> &HashMap<(State, char), State> {
        &self.transitions
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
        for symbol in alphabet.chars() {
            match self.alphabet.insert(symbol) {
                true => (),
                false => {
                    let error = format!("duplicate symbol '{}' in alphabet", symbol);
                    return Err(error);
                }
            }
        }

        // read initial state
        let initial_state = match lines.next() {
            Some(initial_state) => initial_state,
            None => {
                let error = format!("invalid finite automaton file: missing initial state");
                return Err(error);
            }
        };
        self.initial_state = match initial_state.parse::<State>() {
            Ok(initial_state) => initial_state,
            Err(e) => {
                let error = format!("invalid initial state: {}", e.to_string());
                return Err(error);
            }
        };

        // add initial state to set of states
        self.states.insert(self.initial_state);

        // read final states
        let final_states = match lines.next() {
            Some(final_states) => final_states,
            None => {
                let error = format!("invalid finite automaton file: missing final states");
                return Err(error);
            }
        };
        for final_state in final_states.split_whitespace() {
            let final_state = match final_state.parse::<State>() {
                Ok(final_state) => final_state,
                Err(e) => {
                    let error = format!("invalid final state '{}': {}", final_state, e.to_string());
                    return Err(error);
                }
            };

            match self.final_states.insert(final_state) {
                true => (),
                false => {
                    let error = format!("duplicate final state '{}'", final_state);
                    return Err(error);
                }
            }

            // add final state to set of states
            self.states.insert(final_state);
        }

        // read transitions
        for line in lines {
            let mut parts: Vec<&str> = line.split(" ").collect();
            // allow whitespace symbol in transitions
            if parts.len() == 4 {
                if parts[1].is_empty() && parts[2].is_empty() {
                    parts.remove(1);
                    parts[1] = " ";
                }
            }

            if parts.len() != 3 {
                let err = format!("invalid transition: {}", line);
                return Err(err);
            }

            let start_state = match parts[0].parse::<State>() {
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
                let error = format!(
                    "character '{}' missing from alphabet for transition '{}'",
                    symbol, line
                );
                return Err(error);
            }

            let end_state = match parts[2].parse::<State>() {
                Ok(end_state) => end_state,
                Err(e) => {
                    let error = format!("invalid end state '{}': {}", parts[2], e.to_string());
                    return Err(error);
                }
            };

            // check if transition already exists
            if let Some(_) = self.transitions.get(&(start_state, symbol)) {
                let error = format!("duplicate transition key for transition '{}'", line);
                return Err(error);
            }

            self.transitions.insert((start_state, symbol), end_state);

            // add start and end states to set of states
            self.states.insert(start_state);
            self.states.insert(end_state);
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
}
