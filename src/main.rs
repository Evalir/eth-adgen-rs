use k256::{ecdsa::SigningKey, elliptic_curve::sec1::ToEncodedPoint, PublicKey};
use tiny_keccak::{Hasher, Keccak};

pub struct Account {
    // Raw private key
    pub private_key: Vec<u8>,
    // Raw, uncompressed public key (containing the 0x04 lead byte)
    pub public_key: Vec<u8>,
    // Ethereum address
    pub address: Vec<u8>,
}

pub fn keccak256<S>(bytes: S) -> [u8; 32]
where
    S: AsRef<[u8]>,
{
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes.as_ref());
    hasher.finalize(&mut output);
    output
}

pub fn generate_random() -> Account {
    let mut rng = rand::thread_rng();

    let private_key = SigningKey::random(&mut rng);
    let public_key = PublicKey::from(&private_key.verifying_key()).to_encoded_point(false);
    let public_key = public_key.as_bytes();
    // make sure to check the leading byte is 0x04 for the uncompressed public key
    debug_assert_eq!(0x04, public_key[0]);
    // then skip it for address calculation
    let addr = keccak256(&public_key[1..]);

    Account {
        private_key: private_key.to_bytes().to_vec(),
        public_key: public_key.to_vec(),
        address: addr[12..].to_vec(),
    }
}

fn main() -> eyre::Result<()> {
    let account = generate_random();
    println!("private key: 0x{}", hex::encode(account.private_key));
    println!("public key: 0x{}", hex::encode(account.public_key));
    println!("addr: 0x{}", hex::encode(account.address));

    Ok(())
}
