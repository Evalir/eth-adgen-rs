use std::str::FromStr;

use ethers::core::types::*;
use ethers::signers::coins_bip39::{English, Mnemonic};
use ethers::utils::keccak256;

mod util;

#[derive(Debug, Default)]
pub struct Pocketh {}

impl Pocketh {
    pub fn new() -> Self {
        Self {}
    }

    /// Generates a random mnemonic phrase that can then be used to generate accounts.
    ///
    /// ```no_run
    /// use pocketh::Pocketh;
    ///
    /// fn foo() -> eyre::Result<()> {
    ///     let mnemonic = Pocketh::generate_random_phrase();
    ///     println!("{}", mnemonic);
    ///     Ok(())
    /// }
    /// ````
    pub fn generate_random_phrase() -> String {
        let mut rng = rand::thread_rng();

        let mnemonic = Mnemonic::<English>::new(&mut rng);

        mnemonic.to_phrase().unwrap()
    }

    /// Converts from wei, to a different denomination (gwei, ether)
    ///
    /// ```no_run
    /// use pocketh::Pocketh;
    ///
    /// fn foo() -> eyre::Result<()> {
    ///     let wei = 1;
    ///     let gwei = Pocketh::from_wei(1.into(), "gwei".to_string())?; // 0.000000001
    ///     let eth = Pocketh::from_wei(1.into(), "eth".to_string())?; // 0.000000000000000001
    ///     println!("gwei: {}", gwei);
    ///     println!("eth: {}", eth);
    ///     Ok(())
    /// }
    /// ```
    pub fn from_wei(value: U256, unit: String) -> eyre::Result<String> {
        Ok(match &unit[..] {
            "gwei" => ethers::core::utils::format_units(value, 9),
            "eth" | "ether" => ethers::core::utils::format_units(value, 18),
            _ => ethers::core::utils::format_units(value, 18),
        }?)
    }

    /// Converts to wei, from a different denomination (gwei, ether)
    ///
    /// ```no_run
    /// use pocketh::Pocketh;
    ///
    /// fn foo() -> eyre::Result<()> {
    ///     let wei = 1;
    ///     let gwei = Pocketh::to_wei(1.into(), "gwei".to_string())?; // 1000000000
    ///     let eth = Pocketh::to_wei(1.into(), "eth".to_string())?; // 1000000000000000000
    ///     println!("gwei: {}", gwei);
    ///     println!("eth: {}", eth);
    ///     Ok(())
    /// }
    /// ```
    pub fn to_wei(value: f64, unit: String) -> eyre::Result<String> {
        let val = value.to_string();
        Ok(match &unit[..] {
            "gwei" => ethers::core::utils::parse_units(val, 9),
            "eth" | "ether" => ethers::core::utils::parse_units(val, 18),
            _ => ethers::core::utils::parse_units(val, 18),
        }?
        .to_string())
    }

    /// Calculates the selector from a function signature
    ///
    /// ```no_run
    /// use pocketh::Pocketh;
    ///
    /// fn foo() -> eyre::Result<()> {
    ///     let fn_sig = "createAndOpen(address,address)";
    ///     let selector = Pocketh::get_selector(fn_sig)?;
    ///
    ///     println!("{}", selector);
    ///
    ///     Ok(())
    /// }
    pub fn get_selector(sig: &str) -> eyre::Result<String> {
        let hashed_sig = keccak256(sig).to_vec();

        Ok(format!("0x{}", hex::encode(&hashed_sig[..4])))
    }

    /// Calculates the keccak256 hash of the provided payload.
    ///
    /// ```no_run
    /// use pocketh::Pocketh;
    ///
    /// fn foo() -> eyre::Result<()> {
    ///     let payload = "vitalik_masternode";
    ///     let hashed_payload = Pocketh::get_hash(payload)?;
    ///
    ///     println!("{}", hashed_payload);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_hash(payload: &str) -> eyre::Result<String> {
        let hashed_payload = keccak256(payload).to_vec();

        Ok(format!("0x{}", hex::encode(hashed_payload)))
    }

    pub fn uint_to_hex(value: usize) -> eyre::Result<String> {
        Ok(format!("{value:#x}"))
    }

    pub fn hex_to_uint(value: &str) -> eyre::Result<usize> {
        Ok(usize::from_str_radix(strip_0x(value), 16)?)
    }
}

fn strip_0x(s: &str) -> &str {
    s.strip_prefix("0x").unwrap_or(s)
}

#[cfg(test)]
mod tests {
    use super::Pocketh;

    #[test]
    fn test_selector() {
        assert_eq!(
            Pocketh::get_selector("createAndOpen(address,address)").unwrap(),
            "0x581f3c50"
        )
    }

    #[test]
    fn test_hash() {
        assert_eq!(
            Pocketh::get_hash("").unwrap(),
            // base keccak256 response
            "0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
        );
        assert_eq!(
            Pocketh::get_hash("cafeconleche").unwrap(),
            "0x1f19fbea2f63e76368ec292dc853b4c51ada1012666af5435995e15e7f564d2d"
        );
    }

    #[test]
    fn test_uint_to_hex() {
        assert_eq!(Pocketh::uint_to_hex(1).unwrap(), "0x1");
        assert_eq!(Pocketh::uint_to_hex(16).unwrap(), "0x10");
    }

    #[test]
    fn test_hex_to_uint() {
        assert_eq!(Pocketh::hex_to_uint("01").unwrap(), 1);
        assert_eq!(Pocketh::hex_to_uint("10").unwrap(), 16);
        assert_eq!(Pocketh::hex_to_uint("0100").unwrap(), 256);
        assert_eq!(Pocketh::hex_to_uint("1000").unwrap(), 4096);
        assert_eq!(Pocketh::hex_to_uint("1000").unwrap(), 4096);
    }
}
