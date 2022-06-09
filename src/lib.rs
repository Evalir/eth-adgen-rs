use ethers::core::types::*;
use ethers::signers::coins_bip39::{English, Mnemonic};
use ethers::utils::keccak256;

mod util;

pub struct Pocketh {}

impl Default for Pocketh {
    fn default() -> Self {
        Self::new()
    }
}

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
    ///     let selector = Pocketh::get_selector("createAndOpen(address,address)")?;
    ///
    ///     println!("{}", selector);
    ///
    ///     Ok(())
    /// }
    pub fn get_selector(sig: &str) -> eyre::Result<String> {
        let hashed_sig = keccak256(sig).to_vec();

        Ok(hex::encode(&hashed_sig[..4]))
    }
}

#[cfg(test)]
mod tests {
    use super::Pocketh;

    #[test]
    fn test_selector() {
        assert_eq!(
            Pocketh::get_selector("createAndOpen(address,address)").unwrap(),
            "581f3c50"
        )
    }
}
