use std::{fs::File, io::Write};

struct ParseTreeNode {
    symbol: String,
    parent: Option<usize>,     // index of the parent node
    sibling: Option<usize>,    // index of the next sibling node
    last_child: Option<usize>, // index of the last child node, used for efficient sibling traversal
}

pub struct ParserOutput {
    nodes: Vec<ParseTreeNode>,
}

impl ParserOutput {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn add_node(&mut self, symbol: Option<String>, parent: Option<&usize>) {
        let symbol = match symbol {
            Some(symbol) => symbol,
            None => String::from("None"),
        };

        // index where the new node will be inserted
        let node_index = self.len();
        let mut parent_index = None;

        // if the new node has a parent, update the parent's last child and sibling
        if let Some(&parent) = parent {
            if let Some(last_sibling) = self.nodes[parent].last_child {
                // update the current last sibling's sibling to point to the new node
                self.nodes[last_sibling].sibling = Some(node_index);
            }

            // update parent's last_child to point to the new node
            self.nodes[parent].last_child = Some(node_index);
            parent_index = Some(parent);
        }

        let node = ParseTreeNode {
            symbol,
            parent: parent_index,
            sibling: None,
            last_child: None,
        };

        // add the new node to the tree
        self.nodes.push(node);
    }

    fn get_symbol(&self, node_index: Option<usize>) -> String {
        let node = match node_index {
            Some(index) => self.nodes.get(index),
            None => None,
        };

        match node {
            Some(node) => String::from(&node.symbol),
            None => String::from("None"),
        }
    }

    pub fn display(&self) {
        println!("Parse Tree:");
        for (index, node) in self.nodes.iter().enumerate() {
            let parent = self.get_symbol(node.parent);
            let parent_index = match node.parent {
                Some(index) => format!(" ({})", index),
                None => String::from(""),
            };

            let sibling = self.get_symbol(node.sibling);
            let sibling_index = match node.sibling {
                Some(index) => format!(" ({})", index),
                None => String::from(""),
            };

            print!(
                "Node {}: Symbol = {}, Parent = {}{}, Sibling = {}{}\n",
                index, node.symbol, parent, parent_index, sibling, sibling_index
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
            let parent = self.get_symbol(node.parent);
            let parent_index = match node.parent {
                Some(index) => format!(" ({})", index),
                None => String::from(""),
            };

            let sibling = self.get_symbol(node.sibling);
            let sibling_index = match node.sibling {
                Some(index) => format!(" ({})", index),
                None => String::from(""),
            };

            output.push_str(&format!(
                "Node {}: Symbol = {}, Parent = {}{}, Sibling = {}{}\n",
                index, node.symbol, parent, parent_index, sibling, sibling_index
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
