use ethereum_types::{Address, Public, Secret};
use k256::{ecdsa::SigningKey, elliptic_curve::sec1::ToEncodedPoint, PublicKey};
use tiny_keccak::{Hasher, Keccak};

pub struct Account {
    // Raw private key
    pub private_key: Secret,
    // Raw, public key (stripped of the 0x04 lead byte)
    pub public_key: Public,
    // Ethereum address
    pub address: Address,
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
    // make sure to check the leading byte is 0x04 for uncompressed coordinates
    debug_assert_eq!(0x04, public_key[0]);
    // then skip it for address calculation
    let addr = keccak256(&public_key[1..]);

    Account {
        private_key: Secret::from_slice(&private_key.to_bytes()),
        public_key: Public::from_slice(&public_key[1..]),
        address: Address::from_slice(&addr[12..]),
    }
}

fn main() -> eyre::Result<()> {
    let account = generate_random();
    println!("private key: 0x{}", hex::encode(account.private_key));
    println!("public key: 0x{}", hex::encode(account.public_key));
    println!("addr: 0x{}", hex::encode(account.address));

    Ok(())
}
