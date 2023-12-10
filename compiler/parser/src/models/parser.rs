use super::grammar::Grammar;
use super::output::ParserOutput;
use hash_map::HashMap;
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

    pub fn parse(&self, tokens: &Vec<Token>) -> Result<ParserOutput, String> {
        // TODO: implement the output node adding logic
        // TODO: find a way to break out of StatementList
        let mut input = tokens.clone();
        input.push(Token::new(TokenKind::EOF, EOF_TOKEN));

        let mut stack = vec![
            String::from(EOF_TOKEN),
            String::from(self.grammar.get_start_symbol()),
        ];

        let output = ParserOutput::new();
        while let Some(stack_top) = stack.last() {
            if stack_top == EOF_TOKEN
                && input
                    .first()
                    .map_or(false, |t| t.get_kind() == TokenKind::EOF)
            {
                // successfully parsed
                return Ok(output);
            }

            let current_token = &input[0];
            let token_symbol = match current_token.get_kind() {
                TokenKind::Identifier => IDENTIFIER,
                TokenKind::Literal | TokenKind::Number | TokenKind::Char | TokenKind::String => {
                    CONSTANT
                }
                _ => current_token.get_inner(),
            };

            println!(
                "stack_top: {:?} | current_token: {:?}",
                stack_top, current_token
            );
            match (stack_top.as_str(), &current_token.get_kind()) {
                (_, TokenKind::EOF) | (EOF_TOKEN, _) => {
                    self.print_stack_trace(&stack);

                    let error = format!("Parse error: unexpected end of input");
                    return Err(error);
                }
                (top, _) if self.grammar.get_terminals().contains(top) => {
                    if top == token_symbol {
                        stack.pop(); // match terminal
                        input.remove(0);
                        // TODO: add terminal node to output
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
                        stack.pop(); // pop non-terminal

                        // TODO: add non-terminal node to output

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

        let error = format!("Parse error: incomplete input");
        Err(error)
    }

    fn print_stack_trace(&self, stack: &Vec<String>) {
        println!("\nStack trace:");
        for symbol in stack {
            println!("{:?}", symbol);
        }
    }
}
