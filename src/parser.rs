use serde_json::Value;
use crate::config;

pub fn print_tree(nodes: &[Value], parent_id: u32, level: u32) {
    let children = nodes.iter().filter(|n| n[config::PARENT_ID].as_u64().unwrap() == parent_id as u64);
    for (_, child) in children.enumerate() {
        // Print vertical line and indentation
        for _ in 0..level {
            print!("├───  ");
        }
        if level > 0 {
            print!("└───  ");
        }
        // println!("{}", COUNT);
        println!("{} (ID: {}) (Level: {})",
                 child[config::NODE_NAME_EN].as_str().unwrap(),
                 child[config::ID].as_u64().unwrap(),
                 level);
        // Recurse to children with increased level
        print_tree(nodes, child[config::ID].as_u64().unwrap() as u32, level + 1);
    }
}
