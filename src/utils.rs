use crate::accounts::{miner_address, operator_address};
use crate::keys::p2tr_address_from_public_key;
use crate::param::ProviderParams;
use crate::BitcoinRpcClient;
use bitcoin::consensus::encode;
use bitcoin::{Address, Amount, Network, PublicKey, Transaction, XOnlyPublicKey};
use bitcoincore_rpc::bitcoincore_rpc_json::Utxo;
use std::str::FromStr;

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

    pub fn send_utxo_to_address(&self, address: &Address, amount: Amount, utxo_num: usize) {
        for _ in 0..utxo_num {
            let _txid = self.send_to_address(address, amount).unwrap();
        }
        self.gen_regtest_block().unwrap();
    }

    pub fn select_utxo(
        &self,
        public_key: &PublicKey,
        target_amount: Amount,
        network: Network,
    ) -> anyhow::Result<Utxo> {
        let address = p2tr_address_from_public_key(*public_key, network);

        println!("address: {:?}", address.to_string());
        let unspent_utxo = {
            let x_operator_pubkey: XOnlyPublicKey = (*public_key).into();
            let tx_out_set = self.scan_tx_out_set_blocking(&x_operator_pubkey)?;
            tx_out_set.unspents
        };
        println!("unspend_utxos: len {:?}", unspent_utxo.len());
        if unspent_utxo.is_empty() {
            self.send_utxo_to_address(&address, target_amount, 10);
            anyhow::bail!(
                "no utxo avaliable, auto send 20*0.1BTC utxos to it, please rerun, addr: {}",
                address
            );
        }
        let avaliable_utxo = unspent_utxo
            .into_iter()
            .filter(|utxo| utxo.amount >= target_amount)
            .collect::<Vec<_>>();
        if avaliable_utxo.is_empty() {
            self.send_utxo_to_address(&address, target_amount, 10);
            anyhow::bail!(
                "no utxo amount >={}BTC,please retry, addr: {}, ",
                target_amount.to_btc(),
                address
            );
        }

        Ok(avaliable_utxo.first().unwrap().clone())
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
