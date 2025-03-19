use crate::accounts::{miner_address, operator_address};
use crate::param::ProviderParams;
use crate::BitcoinRpcClient;
use bitcoin::consensus::encode;
use bitcoin::{Address, Amount, Network, Transaction};

impl BitcoinRpcClient {
    pub fn gen_regtest_block(&self) -> anyhow::Result<()> {
        let addr = miner_address();
        self.generate_to_address(1, &addr)?;

        Ok(())
    }

    pub fn broadcast_on_regtest(&self, tx: &Transaction) {
        let tx_hex = encode::serialize_hex(&tx);
        let tx_weight = tx.weight().to_wu();
        let compute_txid = tx.compute_txid();
        println!(
            "broadcast txid:{:?}, tx_weight:{tx_weight}, tx_hex: {:?}",
            compute_txid, tx_hex
        );
        let txid = self.post_tx(tx_hex).unwrap();
        assert_eq!(txid, compute_txid);
        println!("Successfully broadcast tx, txid: {:?}", txid);
    }

    pub fn send_to_operator_address(&self, amount: Amount) {
        let operator_addr = operator_address();
        let min_amount = Amount::from_btc(0.1).unwrap();
        let min_amount = if amount > min_amount {
            amount
        } else {
            min_amount
        };

        for _ in 0..20 {
            let _txid = self.send_to_address(&operator_addr, min_amount).unwrap();
        }
        self.gen_regtest_block().unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::param::ProviderParams;
    #[test]
    fn test_gen_regtest_block() {
        let ctx = ProviderParams::local();
        let bitcoin_rpc_client = BitcoinRpcClient::new(
            &format!("{}/wallet/benefactor", ctx.bitcoin_url()),
            &ctx.bitcoin_username(),
            &ctx.bitcoin_password(),
        )
        .expect("Failed to create bitcoin rpc client");

        bitcoin_rpc_client.gen_regtest_block().unwrap();
    }
}
