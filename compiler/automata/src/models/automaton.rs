use hash_map::HashMap;
use std::collections::HashSet as Set;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

use super::state::{NewState, State};
type InputLine = Option<Result<String, io::Error>>;

pub struct Automaton {
    alphabet: Set<char>,
    states: Set<State>,
    initial_state: State,
    final_states: Set<State>,
    transitions: HashMap<(State, char), State>,

    used_states: Set<State>,
}

impl Automaton {
    pub fn new(file_path: &str) -> Result<Self, String> {
        let mut automaton = Self {
            alphabet: Set::new(),
            states: Set::new(),
            initial_state: State::new(),
            final_states: Set::new(),
            transitions: HashMap::new(),

            used_states: Set::new(),
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

    fn parse_file(&mut self, file_path: &str) -> Result<(), String> {
        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                let error = format!("could not open finite automaton file: {}", e.to_string());
                return Err(error);
            }
        };
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // parse alphabet
        match self.parse_alphabet(lines.next()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        // parse states
        match self.parse_states(lines.next()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        // parse initial state
        match self.parse_initial_state(lines.next()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        // parse final states
        match self.parse_final_states(lines.next()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        // parse transitions
        match self.parse_transitions(&mut lines) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        // check if the automaton is consistent
        // i.e. all states in the set of states are used in transitions
        match self.consistency_check() {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    // TODO: move this function to the utils project
    fn extract_line_data(&self, line: InputLine) -> Option<String> {
        // we will ignore the errors from the reader in this function
        // because the lack of data will be caught in the parse functions
        // i.e. parse_alphabet, parse_states, etc.

        match line {
            Some(line) => match line {
                Ok(line) => Some(line),
                Err(_) => None,
            },
            None => None,
        }
    }

    // TODO: move this function to the utils project
    fn get_next_line(&self, lines: &mut Lines<BufReader<File>>) -> Result<Option<String>, String> {
        // we are no longer ignoring errors from the reader in this function
        // because the lack of data is a serious error that should be handled
        // i.e. a missing line breaks the entire definition of the automaton

        match lines.next() {
            Some(line) => match line {
                Ok(line) => Ok(Some(line)),
                Err(e) => {
                    let error = format!("could not read finite automaton file: {}", e.to_string());
                    Err(error)
                }
            },
            None => Ok(None),
        }
    }

    fn parse_alphabet(&mut self, alphabet: InputLine) -> Result<(), String> {
        // check if alphabet is missing
        let alphabet = match self.extract_line_data(alphabet) {
            Some(alphabet) => alphabet,
            None => {
                let error = format!("invalid finite automaton file: missing alphabet");
                return Err(error);
            }
        };

        // add each symbol to the alphabet
        for symbol in alphabet.chars() {
            match self.alphabet.insert(symbol) {
                true => (),
                false => {
                    let error = format!("duplicate symbol '{}' in alphabet", symbol);
                    return Err(error);
                }
            }
        }

        Ok(())
    }

    fn parse_states(&mut self, states: InputLine) -> Result<(), String> {
        // check if set of states is missing
        let states = match self.extract_line_data(states) {
            Some(states) => states,
            None => {
                let error = format!("invalid finite automaton file: missing set of states");
                return Err(error);
            }
        };

        // add each state to the set of states
        for state in states.split_whitespace() {
            // parse state
            let state = match state.parse::<State>() {
                Ok(state) => state,
                Err(e) => {
                    let error = format!("invalid state '{}': {}", state, e.to_string());
                    return Err(error);
                }
            };

            // insert state if it doesn't already exist
            match self.states.insert(state) {
                true => (),
                false => {
                    let error = format!("duplicate state '{}' in set of states", state);
                    return Err(error);
                }
            }
        }

        Ok(())
    }

    fn parse_initial_state(&mut self, initial_state: InputLine) -> Result<(), String> {
        // check if initial state is missing
        let initial_state = match self.extract_line_data(initial_state) {
            Some(initial_state) => initial_state,
            None => {
                let error = format!("invalid finite automaton file: missing initial state");
                return Err(error);
            }
        };

        // parse initial state
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

        self.used_states.insert(self.initial_state);
        Ok(())
    }

    fn parse_final_states(&mut self, final_states: InputLine) -> Result<(), String> {
        // check if final states are missing
        let final_states = match self.extract_line_data(final_states) {
            Some(final_states) => final_states,
            None => {
                let error = format!("invalid finite automaton file: missing final states");
                return Err(error);
            }
        };

        // add each final state to the set of final states
        for final_state in final_states.split_whitespace() {
            // parse final state
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
                true => {
                    self.used_states.insert(final_state);
                }
                false => {
                    let error = format!("duplicate final state '{}'", final_state);
                    return Err(error);
                }
            }
        }

        Ok(())
    }

    fn parse_transitions(
        &mut self,
        transitions: &mut Lines<BufReader<File>>,
    ) -> Result<(), String> {
        loop {
            // get the next line from the reader
            let line = match self.get_next_line(transitions) {
                Ok(line) => line,
                Err(e) => return Err(e),
            };

            // check if the end of the file has been reached
            let line = match line {
                Some(line) => line,
                None => break,
            };

            let mut parts: Vec<&str> = line.split(" ").collect();
            // allow whitespace symbol in transitions
            if parts.len() == 4 {
                if parts[1].is_empty() && parts[2].is_empty() {
                    parts.remove(1);
                    parts[1] = " ";
                }
            }
            // check if transition is invalid
            if parts.len() != 3 {
                let error = format!("invalid transition: {}", line);
                return Err(error);
            }

            // parse start state
            let start_state = match parts[0].parse::<State>() {
                Ok(start_state) => start_state,
                Err(e) => {
                    let error = format!("invalid start state '{}': {}", parts[0], e.to_string());
                    return Err(error);
                }
            };
            // check if start state is in set of states
            if !self.states.contains(&start_state) {
                let error = format!("start state '{}' not in set of states", start_state);
                return Err(error);
            }

            // parse symbol
            if parts[1].len() != 1 {
                let error = format!("invalid symbol '{}' for transition '{}'", parts[1], line);
                return Err(error);
            }
            // check if symbol is empty
            let symbol = match parts[1].chars().next() {
                Some(symbol) => symbol,
                None => {
                    let error = format!("missing symbol for transition '{}'", line);
                    return Err(error);
                }
            };
            // check if symbol is in alphabet
            if !self.alphabet.contains(&symbol) {
                let error = format!(
                    "character '{}' missing from alphabet for transition '{}'",
                    symbol, line
                );
                return Err(error);
            }

            // parse end state
            let end_state = match parts[2].parse::<State>() {
                Ok(end_state) => end_state,
                Err(e) => {
                    let error = format!("invalid end state '{}': {}", parts[2], e.to_string());
                    return Err(error);
                }
            };
            // check if end state is in set of states
            if !self.states.contains(&end_state) {
                let error = format!("end state '{}' not in set of states", end_state);
                return Err(error);
            }

            // check if transition for (start_state, symbol) already exists
            if self.transitions.get(&(start_state, symbol)).is_some() {
                let error = format!("duplicate transition key for transition '{}'", line);
                return Err(error);
            }

            // insert transition
            self.transitions.insert((start_state, symbol), end_state);
            self.used_states.insert(start_state);
            self.used_states.insert(end_state);
        }

        // check if there are no transitions
        if self.transitions.size() == 0 {
            let error = format!("invalid finite automaton file: missing transitions");
            return Err(error);
        }

        Ok(())
    }

    fn consistency_check(&self) -> Result<(), String> {
        // check if all states in the set of states are used in transitions
        for state in &self.states {
            if !self.used_states.contains(state) {
                let error = format!("unused state '{}' in the set of states", state);
                return Err(error);
            }
        }

        Ok(())
    }
}
