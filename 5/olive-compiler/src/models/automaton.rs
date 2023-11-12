use hash_map::HashMap;
use std::collections::HashSet as Set;

use super::state::{NewState, State};

pub struct Automaton {
    alphabet: Set<char>,
    states: Set<State>,
    initial_state: State,
    final_states: Set<State>,
    transitions: HashMap<(State, char), State>,
}

impl Automaton {
    pub fn new(file_path: &str) -> Result<Self, String> {
        let mut automaton = Self {
            alphabet: Set::new(),
            states: Set::new(),
            initial_state: State::new(),
            final_states: Set::new(),
            transitions: HashMap::new(),
        };

        match automaton.parse_file(file_path) {
            Ok(_) => Ok(automaton),
            Err(e) => Err(e),
        }
    }

    pub fn get_alphabet(&self) -> &Set<char> {
        &self.alphabet
    }

    pub fn get_states(&self) -> &Set<State> {
        &self.states
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

        // read states
        let states = match lines.next() {
            Some(states) => states,
            None => {
                let error = format!("invalid finite automaton file: missing set of states");
                return Err(error);
            }
        };
        for state in states.split_whitespace() {
            let state = match state.parse::<State>() {
                Ok(state) => state,
                Err(e) => {
                    let error = format!("invalid state '{}': {}", state, e.to_string());
                    return Err(error);
                }
            };

            match self.states.insert(state) {
                true => (),
                false => {
                    let error = format!("duplicate state '{}' in set of states", state);
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
                let error = format!(
                    "invalid initial state '{}': {}",
                    initial_state,
                    e.to_string()
                );
                return Err(error);
            }
        };

        // check if initial state is in set of states
        if !self.states.contains(&self.initial_state) {
            let error = format!("initial state '{}' not in set of states", initial_state);
            return Err(error);
        }

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

            // check if final state is in set of states
            if !self.states.contains(&final_state) {
                let error = format!("final state '{}' not in set of states", final_state);
                return Err(error);
            }

            // insert final state if it doesn't already exist
            match self.final_states.insert(final_state) {
                true => (),
                false => {
                    let error = format!("duplicate final state '{}'", final_state);
                    return Err(error);
                }
            }
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
            if !self.states.contains(&start_state) {
                let error = format!("start state '{}' not in set of states", start_state);
                return Err(error);
            }

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
            if !self.states.contains(&end_state) {
                let error = format!("end state '{}' not in set of states", end_state);
                return Err(error);
            }

            // check if transition for (start_state, symbol) already exists
            if self.transitions.get(&(start_state, symbol)).is_some() {
                let error = format!("duplicate transition key for transition '{}'", line);
                return Err(error);
            }

            self.transitions.insert((start_state, symbol), end_state);
        }

        // check if there are no transitions
        if self.transitions.size() == 0 {
            let error = format!("invalid finite automaton file: missing transitions");
            return Err(error);
        }

        // extra consistency check below

        // check if all states in the set of states are used in transitions
        for state in &self.states {
            let mut found = false;
            for ((start_state, _), end_state) in &self.transitions {
                if start_state == state || end_state == state {
                    found = true;
                    break;
                }
            }

            if !found {
                let error = format!("unused state '{}' in set of states", state);
                return Err(error);
            }
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
