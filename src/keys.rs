use bitcoin::key::{Secp256k1, TapTweak};
use bitcoin::{secp256k1, Address, Network, PublicKey, ScriptBuf, XOnlyPublicKey};

pub fn tweaked_public_key(public_key: &PublicKey) -> Vec<u8> {
    let secp = bitcoin::secp256k1::Secp256k1::new();
    let x_only = XOnlyPublicKey::from(public_key.clone());
    hex::decode(x_only.tap_tweak(&secp, None).0.to_string())
        .expect("failed to decode tweaked public key")
}
pub fn p2tr_address_from_public_key(public_key: PublicKey, network: Network) -> Address {
    let secp = secp256k1::Secp256k1::new();
    let internal_key = public_key.into();
    Address::p2tr(&secp, internal_key, None, network)
}
