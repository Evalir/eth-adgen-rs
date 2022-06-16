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
    //GetSelector {
    //#[clap(short, long)]
    //signature: String,
    //},
    //GetHash {
    //#[clap(short, long)]
    //payload: String,
    //},
    //FromWei {
    //#[clap(short, long)]
    //value: U256,
    //unit: String,
    //},
    //ToWei {
    //#[clap(short, long)]
    //value: f64,
    //unit: String,
    //},
    //UintToHex {
    //value: usize,
    //},
    //HexToUint {
    //value: String,
    //},
    //StringToHex {
    //value: String,
    //},
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
        Some(Commands::GetMatchingSelector {
            selector,
            args,
            prefix,
            rnd_len,
        }) => {
            println!(
                "{}",
                pocketh::Pocketh::get_matching_selector(selector, args, prefix, *rnd_len).unwrap()
            );
        }
        None => {}
    }

    Ok(())
}
