use clap::{Parser, Subcommand};

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
