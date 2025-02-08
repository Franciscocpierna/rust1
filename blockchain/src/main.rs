mod blockchain;
use blockchain::blockchain::Blockchain;
// use crate::blockchain::{Blockchain, BlockchainIter};
// use std::result::Result;
pub type Result<T> = std::result::Result<T, failure::Error>;
mod cli;
use cli::Cli;

fn main() -> Result<()> {
    // Criar uma nova instância da blockchain
    let cli = Cli::new();
    let _ = cli?.run();


    Ok(())
}
