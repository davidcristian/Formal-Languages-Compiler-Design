use super::grammar::Grammar;
use super::output::ParserOutput;
use hash_map::{HashMap, Table};
use scanner::{Token, TokenKind, EOF_TOKEN};

const IDENTIFIER: &str = "Identifier";
const CONSTANT: &str = "Constant";

// maps a pair (non-terminal, terminal) to a production
type ParsingTable = HashMap<(String, String), String>;

pub struct LL1Parser {
    grammar: Grammar,
    parsing_table: ParsingTable,
}

impl LL1Parser {
    pub fn new(grammar: Grammar) -> Self {
        let mut parser = Self {
            grammar,
            parsing_table: HashMap::new(),
        };

        parser.construct_parsing_table();
        parser
    }

    pub fn get_parsing_table(&self) -> &ParsingTable {
        &self.parsing_table
    }

    fn construct_parsing_table(&mut self) {
        for (non_terminal, productions) in self.grammar.get_productions() {
            for production in productions {
                // split the production into symbols
                let symbols = production.split_whitespace();

                // track if previous symbols had ε in their FIRST set
                let mut should_follow = true;

                for symbol in symbols {
                    // compute FIRST set for the symbol
                    let first_set = self.grammar.first(symbol);

                    // populate the parsing table
                    for first_symbol in &first_set {
                        if first_symbol != "ε" {
                            self.parsing_table.insert(
                                (String::from(non_terminal), String::from(first_symbol)),
                                String::from(production),
                            );
                        }
                    }

                    // break the loop if ε is not in the FIRST set of the current symbol
                    if !first_set.contains("ε") {
                        should_follow = false;
                        break;
                    }
                }

                // if all symbols had ε, add production for each symbol in FOLLOW set
                if should_follow {
                    let follow_set = self.grammar.follow(non_terminal);
                    for follow_symbol in &follow_set {
                        self.parsing_table.insert(
                            (String::from(non_terminal), String::from(follow_symbol)),
                            String::from(production),
                        );
                    }
                }
            }
        }
    }

    pub fn parse(
        &self,
        tokens: &Vec<Token>,
        identifiers: &Table<String>,
        constants: &Table<String>,
    ) -> Result<ParserOutput, String> {
        // TODO: find a way to break out of StatementList (&& on line 96)
        let mut input = tokens.clone();
        input.push(Token::new(TokenKind::EOF, EOF_TOKEN));

        let mut stack = vec![
            String::from(EOF_TOKEN),
            String::from(self.grammar.get_start_symbol()),
        ];

        let mut parent_stack = Vec::new(); // indices of parent nodes
        let mut output = ParserOutput::new();

        while let Some(stack_top) = stack.last() {
            if stack_top == EOF_TOKEN
                || input
                    .first()
                    .map_or(false, |t| t.get_kind() == TokenKind::EOF)
            {
                // successfully parsed
                return Ok(output);
            }

            // get the current token
            let current_token = match input.first() {
                Some(token) => token,
                None => {
                    self.print_stack_trace(&stack);
                    return Err(String::from("Parse error: could not get current token"));
                }
            };

            // get the terminal symbol value from the token
            let token_symbol = match current_token.get_kind() {
                TokenKind::Identifier => IDENTIFIER,
                TokenKind::Constant => CONSTANT,
                _ => current_token.get_inner(),
            };

            // get the value of the token from the identifier or constant table
            let mut token_value = match current_token.get_kind() {
                TokenKind::Identifier => identifiers.get(&current_token.value()).cloned(),
                TokenKind::Constant => constants.get(&current_token.value()).cloned(),
                _ => None,
            };
            // if the token is not an identifier or constant, use the representation
            if token_value.is_none() {
                token_value = Some(String::from(current_token.get_inner()));
            }

            match (stack_top.as_str(), &current_token.get_kind()) {
                (_, TokenKind::EOF) | (EOF_TOKEN, _) => {
                    self.print_stack_trace(&stack);
                    return Err(String::from("Parse error: unexpected end of input stream"));
                }
                (top, _) if self.grammar.get_terminals().contains(top) => {
                    if top == token_symbol {
                        stack.pop(); // match terminal
                        input.remove(0);

                        // add terminal node to output
                        let parent_index = parent_stack.last();
                        output.add_node(token_value, parent_index);
                    } else {
                        self.print_stack_trace(&stack);

                        let error =
                            format!("Parse error: expected {:?}, found {:?}", top, current_token);
                        return Err(error);
                    }
                }
                (top, _) => {
                    // retrieve the corresponding production from the parsing table
                    if let Some(production) = self
                        .parsing_table
                        .get(&(String::from(top), String::from(token_symbol)))
                    {
                        let non_terminal = stack.pop(); // pop non-terminal

                        // the non-terminal node becomes the new parent
                        // for all subsequent nodes until the end of the production
                        let new_parent = output.len();

                        // add non-terminal node to output
                        let parent_index = parent_stack.last();
                        output.add_node(non_terminal, parent_index);

                        // save the new parent index
                        parent_stack.push(new_parent);

                        // push production onto the stack in reverse order
                        for sym in production.split_whitespace().rev() {
                            if sym != "ε" {
                                stack.push(String::from(sym));
                            }
                        }
                    } else {
                        self.print_stack_trace(&stack);

                        let error = format!(
                            "Parse error: no rule for {:?} with {:?}",
                            top, current_token
                        );
                        return Err(error);
                    }
                }
            }
        }

        self.print_stack_trace(&stack);
        Err(String::from(
            "Parse error: grammar exhausted before token stream",
        ))
    }

    fn print_stack_trace(&self, stack: &Vec<String>) {
        println!("\nStack trace:");
        for symbol in stack {
            println!("{:?}", symbol);
        }
    }
}
