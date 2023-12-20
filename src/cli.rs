use anyhow::Result;

pub trait Cli {
    fn execute(&self) -> Result<()>;
}

