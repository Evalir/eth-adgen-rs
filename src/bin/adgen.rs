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
                let account = eth_adgen::generate_random();
                println!("private key: 0x{}", hex::encode(account.private_key));
                println!("public key: 0x{}", hex::encode(account.public_key));
                println!("addr: 0x{}", hex::encode(account.address));
            }
        }
        None => {}
    }

    Ok(())
}
