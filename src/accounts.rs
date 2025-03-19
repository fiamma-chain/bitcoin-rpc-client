use crate::keys::p2tr_address_from_public_key;
use bitcoin::bip32::Xpriv;
use bitcoin::key::Keypair;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::Address;
use bitcoin::Network::Regtest;
use bitcoin::PublicKey;
use std::str::FromStr;

// mine mine the chain, and transfer the utxo to the user.
// 1. mine:
//      bitcoin-cli -regtest generatetoaddress 200 bcrt1pm3xnyl9mrl9vsehmldwrwq7jjvd0q78qzs2x42wqgu4jl6dvrtaq4uz8tm
// 2. transfer:
//      bitcoin-cli -regtest -rpcwallet=benefactor -named sendtoaddress address="bcrt1pmdx8nnpllj3x750zzfqmjvedv34swuka06vda8qau6csnyx2hq9s6p89qf" amount=0.1 fee_rate=25 subtractfeefromamount=false replaceable=true comment="2 pizzas" comment_to="jeremy" verbose=true
//
const MINER_ADDRESS: &str = "bcrt1pm3xnyl9mrl9vsehmldwrwq7jjvd0q78qzs2x42wqgu4jl6dvrtaq4uz8tm";

// User BTC regtest info:
// -rpcwallet=benefactor
// Address: bcrt1phcnl4zcl2fu047pv4wx6y058v8u0n02at6lthvm7pcf2wrvjm5tqatn90k
const USER_PEGIN_PRIVATE_KEY: &str = "tprv8jzau9CfsdkXMzqWFWSgu7f4z1vRk53yiqYqByfoakSLNFQ4bBuTsrUDLXtKHTPZhp161h49vEJr2zwN92G7ZHLZMFvome2U8GcAqDzVRhW";
const USER_PEGIN_PUBLIC_KEY: &str =
    "02a6ac32163539c16b6b5dbbca01b725b8e8acaa5f821ba42c80e7940062140d19";

// Committee BTC regtest info:
// -rpcwallet=benefactor
// p2tr_addr: bcrt1pzvm4mzqjld5quwmmampte0rzcdx9d9rju0h2k43x8aq44mh4937s6cvsrs
const COMMITTEE_PRIVATE_KEY: &str = "tprv8jzau9CfsdkXUGhjKBGrVVC3wNSLun5hTaRjrwMYwHPP2UyaQiajupRBaowKjqqqkEY6yD2dXvGpHQmA6kmeWKCJV1o6PPSUKspLrtsFkgz";
const COMMITTEE_PUBLIC_KEY: &str =
    "02ab8775b3cfd999a12d13ffab497a3db8cd18bbba04ee8ba62dbe08630eb17a23";

// Operator BTC regtest info:
// -rpcwallet=benefactor
// p2tr_addr: bcrt1pmdx8nnpllj3x750zzfqmjvedv34swuka06vda8qau6csnyx2hq9s6p89qf
const OPERATOR_PRIVATE_KEY: &str = "tprv8jzau9CfsdkXPkVBGi313RjQvsXggNwC4SZEBm3ohYAHQrHvBBG9GrPwMRWmzvB2UgkH7vEEjoMwia8kiY1jo6FzeshAfEw8d95ziJHYSTp";
pub(crate) const OPERATOR_PUBLIC_KEY: &str =
    "0385a34c3603c616afaa9da80ee2f354b8caf0308890193b4083cbdee09f998fd0";

// Challenger BTC regtest info:
// -rpcwallet=benefactor
// Address: bcrt1pcq4l9affnwx6xj733syn7alwqfg2u6auvyxptptrz9727slv52wqjvcrgc
const CHALLENGER_PRIVATE_KEY: &str = "tprv8jzau9CfsdkXRhZaPjiA4oYp7wJ5BoMGpqAG4iLf3hoZijtDVufQVHec1po9jVkbUxMc5K43QtUiKjjBgvsSeh2QFtSv9wHf8J3HWD5T5np";
const CHALLENGER_PUBLIC_KEY: &str =
    "02670a6454aa206ee2584e6cb2b61c4a4fe8bce673414712526471168e28c045b6";

// mock bridge data
const ASSERT_TAPROOT_ADDR: &str =
    "bcrt1pfnc8xq5vmwekmg3tfqg886z0yg4x6nm68p7uhxwal9yxhujnewms93xwk2";

// ERC20 info:
// private key: 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
const EVM_RECEIVE_ADDRESS: &str = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";

pub fn operator_keypair() -> Keypair {
    let secp = Secp256k1::new();
    let sk = Xpriv::from_str(OPERATOR_PRIVATE_KEY).unwrap();
    let sk = sk.private_key;
    Keypair::from_secret_key(&secp, &sk)
}
pub fn operator_address() -> Address {
    let operator_pubkey = PublicKey::from_str(OPERATOR_PUBLIC_KEY).expect("invalid public key");
    // let x_operator_pubkey: XOnlyPublicKey = operator_pubkey.into();
    p2tr_address_from_public_key(operator_pubkey, Regtest)
}

pub fn miner_address() -> Address {
    Address::from_str(MINER_ADDRESS).unwrap().assume_checked()
}
