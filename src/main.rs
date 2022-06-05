use k256::{ecdsa::SigningKey, elliptic_curve::sec1::ToEncodedPoint, PublicKey};
use tiny_keccak::{Hasher, Keccak};

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

fn main() -> eyre::Result<()> {
    let mut rng = rand::thread_rng();

    let private_key = SigningKey::random(&mut rng);
    let public_key = PublicKey::from(&private_key.verifying_key()).to_encoded_point(false);
    let public_key = public_key.as_bytes();
    // make sure to check the leading byte is 0x04 for the uncompressed public key
    debug_assert_eq!(0x04, public_key[0]);
    // then skip it for address calculation
    let addr = keccak256(&public_key[1..]);

    println!("private key: 0x{}", hex::encode(private_key.to_bytes()));
    println!("public key: 0x{}", hex::encode(&public_key[1..]));
    println!("addr: 0x{}", hex::encode(&addr[12..]));

    Ok(())
}
