use crate::models::automaton::Automaton;
use crate::models::state::State;
use crate::utils::input::{get_dfa_folder, get_dfa_path, read_string, read_usize};

pub struct Menu {}

impl Menu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&self) {
        let mut automaton: Option<Automaton> = None;
        let menu_items = vec![
            "Exit",
            "Load automaton",
            "Display details",
            "Verify sequence",
        ];

        self.display_menu(&menu_items, "Main menu");

        loop {
            let choice = read_usize("\nOption:");
            match choice {
                0 => {
                    println!("INFO: Exiting.");
                    break;
                }
                1 => {
                    self.load_dfa(&mut automaton);
                }
                2 => {
                    match self.dfa_submenu(&automaton) {
                        Ok(_) => {
                            // display main menu again
                            self.display_menu(&menu_items, "Main menu");
                        }
                        Err(_) => (),
                    };
                }
                3 => {
                    self.verify_sequence(&automaton);
                }
                _ => println!("ERROR: Invalid option!"),
            }
        }
    }

    fn display_menu(&self, menu: &Vec<&str>, name: &str) {
        println!("\n{}:", name);
        for (index, item) in menu.iter().enumerate() {
            println!("{}. {}", index, item);
        }
    }

    fn load_dfa(&self, automaton: &mut Option<Automaton>) {
        println!(
            "Enter the name of the file from the '{}' folder, without the extension.",
            get_dfa_folder().trim_end_matches('/')
        );
        let file_name = read_string("File name:");
        let file_path = get_dfa_path(&file_name);

        *automaton = match Automaton::new(&file_path) {
            Ok(automaton) => Some(automaton),
            Err(_) => {
                println!("ERROR: Could not load automaton!");
                return;
            }
        };

        println!("INFO: Automaton loaded!");
    }

    fn verify_sequence(&self, automaton: &Option<Automaton>) {
        let automaton = match automaton {
            Some(automaton) => automaton,
            None => {
                println!("INFO: No automaton loaded!");
                return;
            }
        };

        let sequence = read_string("Sequence:");
        if automaton.validate(&sequence) {
            println!("INFO: Sequence accepted!");
        } else {
            println!("INFO: Sequence rejected!");
        }
    }

    fn dfa_submenu(&self, automaton: &Option<Automaton>) -> Result<(), ()> {
        let automaton = match automaton {
            Some(automaton) => automaton,
            None => {
                println!("INFO: No automaton loaded!");
                return Err(());
            }
        };

        let menu_items = vec![
            "Back",
            "Set of states",
            "Alphabet",
            "Transitions",
            "Initial state",
            "Set of final states",
        ];

        self.display_menu(&menu_items, "Display details");

        loop {
            let choice = read_usize("\nOption:");
            match choice {
                0 => {
                    println!("INFO: Returning to the previous menu.");
                    break;
                }
                1 => {
                    // get a copy of the states and sort them
                    let mut states: Vec<State> = automaton.get_states().iter().cloned().collect();
                    states.sort();

                    println!("Set of states: {:?}", &states);
                }
                2 => {
                    // get a copy of the alphabet and sort it
                    let mut alphabet: Vec<char> =
                        automaton.get_alphabet().iter().cloned().collect();
                    alphabet.sort();

                    println!("Alphabet: {:?}", &alphabet);
                }
                3 => {
                    // get a copy of the transitions and sort them
                    let mut transitions: Vec<((State, char), State)> = automaton
                        .get_transitions()
                        .iter()
                        .map(|(key, value)| (*key, *value))
                        .collect();

                    // sort by key = (start_state, symbol)
                    transitions.sort_by(|(key1, _), (key2, _)| key1.cmp(key2));

                    println!("Transitions:");
                    for ((start_state, symbol), end_state) in &transitions {
                        println!("({:?}, {:?}) -> {:?}", start_state, symbol, end_state);
                    }
                }
                4 => {
                    // print the initial state
                    println!("Initial state: {:?}", automaton.get_initial_state());
                }
                5 => {
                    // get a copy of the final states and sort them
                    let mut final_states: Vec<State> =
                        automaton.get_final_states().iter().cloned().collect();
                    final_states.sort();

                    println!("Set of final states: {:?}", &final_states);
                }
                _ => println!("ERROR: Invalid option!"),
            }
        }

        Ok(())
    }
}
