use crate::accounts::{miner_address, operator_address};
use crate::param::ProviderParams;
use crate::BitcoinRpcClient;
use bitcoin::consensus::encode;
use bitcoin::{Address, Amount, Network, Transaction};

pub fn gen_regtest_block() -> anyhow::Result<()> {
    let ctx = ProviderParams::local();
    let bitcoin_rpc_client = BitcoinRpcClient::new(
        &ctx.bitcoin_url(),
        &ctx.bitcoin_username(),
        &ctx.bitcoin_password(),
    )
    .expect("Failed to create bitcoin rpc client");

    let addr = miner_address();
    bitcoin_rpc_client.generate_to_address(1, &addr)?;

    Ok(())
}

pub fn broadcast_on_regtest(tx: &Transaction) {
    let ctx = ProviderParams::local();
    let bitcoin_rpc_client = BitcoinRpcClient::new(
        &ctx.bitcoin_url(),
        &ctx.bitcoin_username(),
        &ctx.bitcoin_password(),
    )
    .expect("Failed to create bitcoin rpc client");
    let tx_hex = encode::serialize_hex(&tx);
    let tx_weight = tx.weight().to_wu();
    let compute_txid = tx.compute_txid();
    println!(
        "broadcast txid:{:?}, tx_weight:{tx_weight}, tx_hex: {:?}",
        compute_txid, tx_hex
    );
    let txid = bitcoin_rpc_client.post_tx(tx_hex).unwrap();
    assert_eq!(txid, compute_txid);
    println!("Successfully broadcast tx, txid: {:?}", txid);
}

pub fn send_to_operator_address(amount: Amount) {
    let ctx = ProviderParams::local();
    let bitcoin_rpc_client = BitcoinRpcClient::new(
        &format!("{}/wallet/benefactor", ctx.bitcoin_url()),
        &ctx.bitcoin_username(),
        &ctx.bitcoin_password(),
    )
    .expect("Failed to create bitcoin rpc client");

    let operator_addr = operator_address();
    let min_amount = Amount::from_btc(0.1).unwrap();
    let min_amount = if amount > min_amount {
        amount
    } else {
        min_amount
    };

    for _ in 0..20 {
        let _txid = bitcoin_rpc_client
            .send_to_address(&operator_addr, min_amount)
            .unwrap();
    }
    gen_regtest_block().unwrap();
}

#[test]
fn test_gen_regtest_block() {
    gen_regtest_block().unwrap();
}
