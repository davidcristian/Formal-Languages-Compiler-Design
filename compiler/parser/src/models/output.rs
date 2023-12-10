use std::{fs::File, io::Write};

struct ParseTreeNode {
    symbol: String,
    position: usize,        // index in the list of tokens
    parent: Option<usize>,  // index of the parent node
    sibling: Option<usize>, // index of the next sibling node
}

pub struct ParserOutput {
    nodes: Vec<ParseTreeNode>,
}

impl ParserOutput {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    // TODO: add a method to add a node to the tree

    pub fn display(&self) {
        println!("Parse Tree:");
        for (index, node) in self.nodes.iter().enumerate() {
            println!(
                "Node {}: Symbol = {}, Position = {}, Parent = {:?}, Sibling = {:?}",
                index, node.symbol, node.position, node.parent, node.sibling
            );
        }
    }

    pub fn write_output(&self, file_path: &str) -> Result<(), String> {
        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(e) => {
                let error = format!("could not create output file: {}", e.to_string());
                return Err(error);
            }
        };

        let mut output = String::new();
        for (index, node) in self.nodes.iter().enumerate() {
            output.push_str(&format!(
                "Node {}: Symbol = {}, Position = {}, Parent = {:?}, Sibling = {:?}\n",
                index, node.symbol, node.position, node.parent, node.sibling
            ));
        }

        match file.write_all(output.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => {
                let error = format!("could not write to output file: {}", e.to_string());
                Err(error)
            }
        }
    }
}
