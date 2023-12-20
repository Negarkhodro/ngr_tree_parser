use crate::{cli::Cli};
use clap::Subcommand;
use crate::menu::TreeNode;

#[derive(Subcommand)]
pub enum TreeCommand {
    /// Manage tree nodes with CLI
    TreeNodes {
        #[command(subcommand)]
        tree_commands: TreeNode,
    },
}

impl Cli for TreeCommand {
    fn execute(&self) -> anyhow::Result<()> {
        match self {
            Self::TreeNodes { tree_commands } => tree_commands.execute(),
        }
    }
}