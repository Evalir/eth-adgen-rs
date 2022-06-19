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
    GetMatchingSelector {
        #[clap(short, long)]
        selector: String,
        #[clap(short, long)]
        args: String,
        #[clap(short, long)]
        prefix: String,
        #[clap(short, long, default_value = "6")]
        rnd_len: usize,
    },
    UintToHex {
        #[clap(short, long)]
        value: usize,
    },
    HexToUint {
        #[clap(short, long)]
        value: String,
    },
    StringToHex {
        #[clap(short, long)]
        value: String,
    },
}

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();
    let pocketh = pocketh::Pocketh::new();

    match &cli.command {
        Some(Commands::GenerateRandomAccount { amount }) => {
            for _ in 0..*amount {
                let mnemonic = pocketh.generate_random_phrase();
                println!("{}", mnemonic);
            }
        }
        Some(Commands::GetMatchingSelector {
            selector,
            args,
            prefix,
            rnd_len,
        }) => {
            println!(
                "{}",
                pocketh
                    .get_matching_selector(selector, args, prefix, *rnd_len)
                    .unwrap()
            );
        }
        Some(Commands::GetHash { payload }) => {
            println!("{}", pocketh.get_hash(&payload)?);
        }
        Some(Commands::FromWei { value, unit }) => {
            println!("{}", pocketh.from_wei(*value, unit.to_string())?);
        }
        Some(Commands::ToWei { value, unit }) => {
            println!("{}", pocketh.to_wei(*value, unit.to_string())?);
        }
        Some(Commands::UintToHex { value }) => {
            println!("{}", pocketh.uint_to_hex(*value)?);
        }
        Some(Commands::HexToUint { value }) => {
            println!("{}", pocketh.hex_to_uint(value)?);
        }
        Some(Commands::StringToHex { value }) => {
            println!("{}", pocketh.hex_to_uint(value)?);
        }
        Some(Commands::GetSelector { signature }) => {
            println!("{}", pocketh.get_selector(signature)?);
        }
        None => {}
    }

    Ok(())
}
