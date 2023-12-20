use std::fs::File;
use std::io::Read;
use crate::cli::Cli;
use clap::Subcommand;
use serde_json::Value;

use crate::parser;
use crate::config;

#[derive(Subcommand)]
pub enum AddTreeNode {
    /// Adding to new node in the tree
    Add {
        /// Add node to the tree with parent_id
        #[arg(long)]
        parent_id: i32
    },
}

#[derive(Subcommand)]
pub enum RemoveTreeNode {
    /// Delete node from the tree
    Remove {
        /// Delete from the tree with ID
        #[arg(long)]
        id: i32
    },
}

#[derive(Subcommand)]
pub enum ListTreeNode {
    /// Show all nodes in the tree
    Show,
}

#[derive(Subcommand)]
pub enum TreeNode {
    /// Show tree nodes
    ShowTree {
        #[command(subcommand)]
        show_nodes: ListTreeNode
    },
    /// Add to tree nodes
    AddTree {
        #[command(subcommand)]
        add_node: AddTreeNode
    },
    /// Remove from tree nodes
    RemoveTree {
        #[command(subcommand)]
        rm_node: RemoveTreeNode
    },
}

fn tree_show() {
    // Open the file
    let mut file = File::open(config::FILEPATH).expect("Failed to open file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Failed to read file");
    let parsed_json: Value = serde_json::from_str(&file_contents).expect("Failed to parse JSON");
    if let Some(nodes) = parsed_json.get(config::PARENT) {
        let nodes = nodes.as_array().unwrap();
        parser::print_tree(nodes, 0, 0);
    }
}

impl Cli for TreeNode {
    fn execute(&self) -> anyhow::Result<()> {
        match self {
            Self::ShowTree {
                show_nodes
            } => {
                match show_nodes {
                    ListTreeNode::Show => {
                        tree_show();
                        Ok(())
                    }
                }
            }
            Self::AddTree {
                add_node
            } => {
                match add_node {
                    AddTreeNode::Add { parent_id} => {
                        println!("Parent ID {}", parent_id);
                        Ok(())
                    }
                }
            }
            Self::RemoveTree {
                rm_node
            } => {
                match rm_node {
                    RemoveTreeNode::Remove {id} => {
                        println!("Removing ID {}", id);
                        Ok(())
                    }
                }
            }
        }
    }
}