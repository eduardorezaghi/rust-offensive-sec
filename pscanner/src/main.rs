pub mod subdomains;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum MyError {
    #[error("Usage: pscanner <domain>")]
    CliUsage,
}

fn main() -> Result<(), anyhow::Error> {
    //...
}
