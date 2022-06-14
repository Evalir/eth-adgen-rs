use clap::{Parser, Subcommand};
use ethers::prelude::U256;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    GenerateRandomAccount {
        #[clap(short, long)]
        amount: usize,
    },
    GetSelector {
        #[clap(short, long)]
        signature: String,
    },
    GetHash {
        #[clap(short, long)]
        payload: String,
    },
    FromWei {
        #[clap(short, long)]
        value: U256,
        unit: String,
    },
    ToWei {
        #[clap(short, long)]
        value: f64,
        unit: String,
    },
    UintToHex {
        value: usize,
    },
    HexToUint {
        value: String,
    },
    StringToHex {
        value: String,
    },
}

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::GenerateRandomAccount { amount }) => {
            for _ in 0..*amount {
                let mnemonic = pocketh::Pocketh::generate_random_phrase();
                println!("{}", mnemonic);
            }
        }
        None => {}
    }

    Ok(())
}
