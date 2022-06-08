fn main() -> eyre::Result<()> {
    let account = eth_adgen::generate_random();
    println!("private key: 0x{}", hex::encode(account.private_key));
    println!("public key: 0x{}", hex::encode(account.public_key));
    println!("addr: 0x{}", hex::encode(account.address));

    Ok(())
}
