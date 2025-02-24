// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

//! NIP06
//!
//! <https://github.com/nostr-protocol/nips/blob/master/06.md>

use alloc::vec;
use alloc::vec::Vec;
use core::fmt;
use core::str::FromStr;

use bip39::Mnemonic;
use bitcoin::bip32::{ChildNumber, DerivationPath, ExtendedPrivKey};
use bitcoin::hashes::hmac::{Hmac, HmacEngine};
use bitcoin::hashes::{sha512, Hash, HashEngine};
#[cfg(feature = "std")]
use bitcoin::secp256k1::rand::rngs::OsRng;
use bitcoin::secp256k1::rand::RngCore;
use bitcoin::secp256k1::{Secp256k1, Signing};
use bitcoin::Network;

#[cfg(feature = "std")]
use crate::SECP256K1;
use crate::{Keys, SecretKey};

const PURPOSE: u32 = 44;
const COIN: u32 = 1237;

/// `NIP06` error
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// BIP32 error
    BIP32(bitcoin::bip32::Error),
    /// BIP39 error
    BIP39(bip39::Error),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BIP32(e) => write!(f, "BIP32: {e}"),
            Self::BIP39(e) => write!(f, "BIP39: {e}"),
        }
    }
}

impl From<bitcoin::bip32::Error> for Error {
    fn from(e: bitcoin::bip32::Error) -> Self {
        Self::BIP32(e)
    }
}

impl From<bip39::Error> for Error {
    fn from(e: bip39::Error) -> Self {
        Self::BIP39(e)
    }
}

/// NIP06 utils
///
/// <https://github.com/nostr-protocol/nips/blob/master/06.md>
pub trait FromMnemonic: Sized {
    /// Error
    type Err;

    /// Derive from BIP-39 mnemonics (ENGLISH wordlist).
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/06.md>
    #[cfg(feature = "std")]
    fn from_mnemonic<S>(mnemonic: S, passphrase: Option<S>) -> Result<Self, Self::Err>
    where
        S: AsRef<str>,
    {
        Self::from_mnemonic_with_account(mnemonic, passphrase, None)
    }

    /// Derive from BIP-39 mnemonics with **custom account** (ENGLISH wordlist).
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/06.md>
    #[cfg(feature = "std")]
    fn from_mnemonic_with_account<S>(
        mnemonic: S,
        passphrase: Option<S>,
        account: Option<u32>,
    ) -> Result<Self, Self::Err>
    where
        S: AsRef<str>,
    {
        Self::from_mnemonic_advanced(mnemonic, passphrase, account, None, None)
    }

    /// Derive from BIP-39 mnemonics with **custom** `account`, `type` and/or `index` (ENGLISH wordlist).
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/06.md>
    #[cfg(feature = "std")]
    fn from_mnemonic_advanced<S>(
        mnemonic: S,
        passphrase: Option<S>,
        account: Option<u32>,
        r#type: Option<u32>,
        index: Option<u32>,
    ) -> Result<Self, Self::Err>
    where
        S: AsRef<str>,
    {
        Self::from_mnemonic_with_ctx(&SECP256K1, mnemonic, passphrase, account, r#type, index)
    }

    /// Derive from BIP-39 mnemonics with **custom account** (ENGLISH wordlist).
    ///
    /// By default `account`, `type` and `index` are set to `0`.
    ///
    /// <https://github.com/nostr-protocol/nips/blob/master/06.md>
    fn from_mnemonic_with_ctx<C, S>(
        secp: &Secp256k1<C>,
        mnemonic: S,
        passphrase: Option<S>,
        account: Option<u32>,
        r#type: Option<u32>,
        index: Option<u32>,
    ) -> Result<Self, Self::Err>
    where
        C: Signing,
        S: AsRef<str>;
}

#[deprecated(since = "0.29.0")]
#[allow(missing_docs)]
pub trait GenerateMnemonic {
    type Err;

    #[cfg(feature = "std")]
    fn generate_mnemonic(word_count: usize) -> Result<Mnemonic, Self::Err>;

    fn generate_mnemonic_with_rng<R>(rng: &mut R, word_count: usize) -> Result<Mnemonic, Self::Err>
    where
        R: RngCore;
}

impl FromMnemonic for Keys {
    type Err = Error;

    fn from_mnemonic_with_ctx<C, S>(
        secp: &Secp256k1<C>,
        mnemonic: S,
        passphrase: Option<S>,
        account: Option<u32>,
        r#type: Option<u32>,
        index: Option<u32>,
    ) -> Result<Self, Self::Err>
    where
        C: Signing,
        S: AsRef<str>,
    {
        // Parse menmonic
        let mnemonic: Mnemonic = Mnemonic::from_str(mnemonic.as_ref())?;

        // Convert mnemonic to seed
        let seed: [u8; 64] = mnemonic
            .to_seed_normalized(passphrase.as_ref().map(|s| s.as_ref()).unwrap_or_default());

        // Derive BIP32 root key
        let root_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)?;

        // Unwrap idx
        let account: u32 = account.unwrap_or_default();
        let _type: u32 = r#type.unwrap_or_default();
        let index: u32 = index.unwrap_or_default();

        // Compose derivation path
        let path: Vec<ChildNumber> = vec![
            ChildNumber::from_hardened_idx(PURPOSE)?,
            ChildNumber::from_hardened_idx(COIN)?,
            ChildNumber::from_hardened_idx(account)?,
            ChildNumber::from_normal_idx(_type)?,
            ChildNumber::from_normal_idx(index)?,
        ];
        let path = DerivationPath::from(path);

        // Derive secret key
        let child_xprv = root_key.derive_priv(secp, &path)?;
        let secret_key = SecretKey::from(child_xprv.private_key);

        // Compose keys
        Ok(Self::new_with_ctx(secp, secret_key))
    }
}

#[allow(deprecated)]
impl GenerateMnemonic for Keys {
    type Err = Error;

    /// Generate new `mnemonic`
    #[cfg(feature = "std")]
    fn generate_mnemonic(word_count: usize) -> Result<Mnemonic, Self::Err> {
        let mut rng = OsRng;
        Self::generate_mnemonic_with_rng(&mut rng, word_count)
    }

    fn generate_mnemonic_with_rng<R>(rng: &mut R, word_count: usize) -> Result<Mnemonic, Self::Err>
    where
        R: RngCore,
    {
        let mut h = HmacEngine::<sha512::Hash>::new(b"nostr");
        let mut os_random = [0u8; 32];
        rng.fill_bytes(&mut os_random);
        h.input(&os_random);
        let entropy: [u8; 64] = Hmac::from_engine(h).to_byte_array();
        let len: usize = word_count * 4 / 3;
        Ok(Mnemonic::from_entropy(&entropy[0..len])?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nip06() {
        let secp = Secp256k1::new();

        let list = vec![
            ("equal dragon fabric refuse stable cherry smoke allow alley easy never medal attend together lumber movie what sad siege weather matrix buffalo state shoot", "06992419a8fe821dd8de03d4c300614e8feefb5ea936b76f89976dcace8aebee"),
            ("leader monkey parrot ring guide accident before fence cannon height naive bean", "7f7ff03d123792d6ac594bfa67bf6d0c0ab55b6b1fdb6249303fe861f1ccba9a"),
            ("what bleak badge arrange retreat wolf trade produce cricket blur garlic valid proud rude strong choose busy staff weather area salt hollow arm fade", "c15d739894c81a2fcfd3a2df85a0d2c0dbc47a280d092799f144d73d7ae78add"),
        ];

        for (mnemonic, expected_secret_key) in list.into_iter() {
            let keys =
                Keys::from_mnemonic_with_ctx(&secp, mnemonic, None, None, None, None).unwrap();
            assert_eq!(
                keys.secret_key().unwrap(),
                &SecretKey::from_str(expected_secret_key).unwrap()
            );
        }
    }
}
