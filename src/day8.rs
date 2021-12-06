use std::collections::VecDeque;

pub fn day8(input_lines: &[String]) -> (String, String) {
    let mut data = input_lines[0].split_ascii_whitespace().map(|datum| datum.parse::<u64>().expect("Non-numeric input"));
    let mut total_metadata = 0u64;
    let mut stack: VecDeque<Node> = VecDeque::new();
    stack.push_back(Node::new());
    let mut root_node: Option<Node> = None;
    while !stack.is_empty() {
        let current_node = stack.back_mut().unwrap();

        if !current_node.header_read {
            current_node.num_pending_child_nodes = data.next().unwrap();
            current_node.num_pending_metadata_entries = data.next().unwrap();
            current_node.header_read = true;
        }

        if current_node.num_pending_child_nodes > 0 {
            let new_node = Node::new();
            stack.push_back(new_node);
        } else {
            for _ in 0..current_node.num_pending_metadata_entries {
                let metadata = data.next().unwrap();
                current_node.metadata_entries.push(metadata);
                total_metadata += metadata;
            }
            current_node.num_pending_metadata_entries = 0;

            // Node is now fully read, update parent and pop it from the stack.
            let child_node = stack.pop_back().unwrap();
            if let Some(parent_node) = stack.back_mut() {
                parent_node.child_nodes.push(child_node);
                parent_node.num_pending_child_nodes -= 1;
            } else {
                // This was the root node, we're done.
                root_node = Some(child_node);
            }
        }
    }

    let part1 = total_metadata.to_string();
    let part2 = root_node.unwrap().value().to_string();
    (part1, part2)
}

#[derive(Default)]
struct Node {
    header_read: bool,
    num_pending_child_nodes: u64,
    num_pending_metadata_entries: u64,
    child_nodes: Vec<Node>,
    metadata_entries: Vec<u64>,
}

impl Node {
    fn new() -> Self {
        Self {
            header_read: false,
            num_pending_child_nodes: 0,
            num_pending_metadata_entries: 0,
            child_nodes: Vec::new(),
            metadata_entries: Vec::new(),
        }
    }

    fn value(&self) -> u64 {
        let mut value = 0u64;
        if self.child_nodes.is_empty() {
            value = self.metadata_entries.iter().sum();
        } else {
            for metadata in self.metadata_entries.iter() {
                let index = *metadata as usize - 1;
                if index < self.child_nodes.len() {
                    value += self.child_nodes[index].value();
                }
            }
        }

        value
    }
}