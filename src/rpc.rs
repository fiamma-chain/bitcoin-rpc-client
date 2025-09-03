use anyhow::anyhow;
use bitcoin::address::NetworkChecked;
use bitcoin::{Address, Amount, XOnlyPublicKey};
use bitcoincore_rpc::bitcoincore_rpc_json::ScanTxOutResult;
use bitcoincore_rpc::json::{
    GetBlockHeaderResult, GetRawTransactionResult, ListUnspentResultEntry,
};
use bitcoincore_rpc::{json, Auth, Client, Error, Result, RpcApi};

#[derive(Debug)]
pub struct BitcoinRpcClient {
    client: Client,
}

impl BitcoinRpcClient {
    pub fn new(url: &str, user: &str, password: &str) -> bitcoincore_rpc::Result<Self> {
        let client =
            Client::new_with_minreq(url, Auth::UserPass(user.to_string(), password.to_string()))?;
        Ok(Self { client })
    }

    pub fn post_tx(&self, tx: String) -> bitcoincore_rpc::Result<bitcoin::Txid> {
        self.client.send_raw_transaction(tx)
    }

    pub fn check_tx(&self, tx: String) -> bitcoincore_rpc::Result<bitcoin::Txid> {
        // check if tx can be post,
        //  if the tx has been posted, it won't be allowed.
        //  or the tx is illegal
        let response = self.client.test_mempool_accept(&[tx.clone()]);
        match response {
            Ok(response) => {
                let check_mempool_accept = response
                    .first()
                    .ok_or({
                        let error_info = format!("test_mempool_accept return empty, tx: {}", tx,);
                        log::error!("{error_info}",);

                        Error::ReturnedError(error_info)
                    })?
                    .to_owned();
                if check_mempool_accept.allowed {
                    Ok(check_mempool_accept.txid)
                } else {
                    let error_info = format!(
                        "test_mempool_accept isn't allowed, error: {:?}",
                        check_mempool_accept.reject_reason.clone()
                    );
                    // txn-already-known: if inputs utxo missing because we already have the tx
                    // txn-already-in-mempool: exact transaction already exists in the mempool.
                    if error_info.contains("txn-already-in-mempool")
                        || error_info.contains("txn-already-known")
                        || error_info.contains("Transaction outputs already in utxo set")
                    {
                        Ok(check_mempool_accept.txid)
                    } else {
                        log::error!("{}", error_info);
                        Err(Error::ReturnedError(error_info))
                    }
                }
            }
            Err(err) => {
                let error_info = format!(
                    "test_mempool_accept invoke failed, tx:{tx}, error: {:?}",
                    err
                );
                log::error!("{}", error_info);
                Err(Error::ReturnedError(error_info))
            }
        }
    }

    pub fn check_and_post_tx(&self, tx: String) -> bitcoincore_rpc::Result<bitcoin::Txid> {
        self.check_tx(tx.clone())?;
        // post tx
        self.post_tx(tx)
    }

    pub fn get_tx(&self, tx_id: bitcoin::Txid) -> bitcoincore_rpc::Result<bitcoin::Transaction> {
        self.client.get_raw_transaction(&tx_id, None)
    }

    pub fn get_tx_info(
        &self,
        tx_id: bitcoin::Txid,
    ) -> bitcoincore_rpc::Result<GetRawTransactionResult> {
        self.client.get_raw_transaction_info(&tx_id, None)
    }

    pub fn get_block_count(&self) -> bitcoincore_rpc::Result<u64> {
        self.client.get_block_count()
    }

    // List unspent transaction outputs, for this api, url must has wallet field, e.g. "http://127.0.0.1:18443/wallet/benefactor".
    pub fn get_unspent(
        &self,
        address: &Address,
        min_confirmation: Option<usize>,
    ) -> bitcoincore_rpc::Result<Vec<ListUnspentResultEntry>> {
        self.client
            .list_unspent(min_confirmation, None, Some(&[address]), Some(true), None)
    }

    pub fn get_block(
        &self,
        block_hash: &bitcoin::BlockHash,
    ) -> bitcoincore_rpc::Result<bitcoin::Block> {
        self.client.get_block(block_hash)
    }

    pub fn get_block_height(&self, block_hash: &bitcoin::BlockHash) -> anyhow::Result<u64> {
        let block = self.client.get_block(block_hash)?;
        block
            .bip34_block_height()
            .map_err(|e| anyhow!("failed to get_block_height {}", e.to_string()))
    }

    pub fn get_block_header_info(
        &self,
        block_hash: &bitcoin::BlockHash,
    ) -> bitcoincore_rpc::Result<GetBlockHeaderResult> {
        self.client.get_block_header_info(block_hash)
    }

    pub fn get_block_midian_time(&self, height: u64) -> bitcoincore_rpc::Result<u64> {
        let fields = vec![json::BlockStatsFields::MedianTime];
        let block_header_info = self.client.get_block_stats_fields(height, &fields)?;

        if block_header_info.median_time.is_none() {
            return Err(bitcoincore_rpc::Error::ReturnedError(format!(
                "get_block_midian_time failed, where block_num is {}",
                height
            )));
        }
        let midian_time = block_header_info.median_time.unwrap();
        Ok(midian_time)
    }
    pub fn get_best_block_midian_time(&self) -> bitcoincore_rpc::Result<u64> {
        let block = self.get_block_count()?;
        self.get_block_midian_time(block)
    }

    pub fn scan_tx_out_set_blocking(
        &self,
        x_only_pubkey: &XOnlyPublicKey,
    ) -> bitcoincore_rpc::Result<ScanTxOutResult> {
        let desc = format!("tr({})", x_only_pubkey);
        let request = json::ScanTxOutRequest::Single(desc);
        self.client.scan_tx_out_set_blocking(&[request])
    }

    pub fn generate_to_address(
        &self,
        block_num: u64,
        address: &Address<NetworkChecked>,
    ) -> bitcoincore_rpc::Result<Vec<bitcoin::BlockHash>> {
        assert!(block_num > 0);
        self.client.generate_to_address(block_num, address)
    }

    pub fn send_to_address(
        &self,
        address: &Address<NetworkChecked>,
        amount: Amount,
    ) -> bitcoincore_rpc::Result<bitcoin::Txid> {
        self.client
            .send_to_address(address, amount, None, None, None, None, None, None)
    }

    pub fn get_blockhash_by_height(&self, height: u64) -> Result<bitcoin::BlockHash> {
        self.client.get_block_hash(height)
    }
}

#[cfg(test)]
mod tests {
    use bitcoin::{Address, Network};
    use bitcoincore_rpc::{Auth, Client, RpcApi};
    use std::str::FromStr;

    #[test]
    fn test_rpc() {
        let url = "http://127.0.0.1:18443/wallet/benefactor";
        let user = "test".to_string();
        let pass = "1234".to_string();
        let rpc = Client::new(&url, Auth::UserPass(user, pass)).unwrap();
        let blockchain_info = rpc.get_blockchain_info();
        println!("blockchain_info: {:?}", blockchain_info);

        // let raw_signed_tx = "0200000000010113953490481ce4c7d8a7df2b3a5544d382a7db72904a5b9bcc2d9645bda3ab9c0000000000ffffffff028096980000000000220020748d118052d6e418922165b03a3191cb70ef216aa65428d6ca8951d20e78bdda98576d2901000000225120be27fa8b1f5278faf82cab8da23e8761f8f9bd5d5ebebbb37e0e12a70d92dd160141759ea221004211674874af3c603316aab7e7ff1e4c8217f224c4104b1ae353e64a00de10fdd44a9f3adc877ec10e2decbfaa80005c4951c433010012d9a50aab0100000000";

        // let resp = rpc.send_raw_transaction(raw_signed_tx);
        // println!("resp: {:?}", resp);
    }

    #[test]
    fn test_utxo() {
        // curl --user test --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getrawtransaction", "params": ["658a4f87dbadd48398c0ecd7072c249a5da0cdff14d898d5bf63f6d8e666d911", true]}' -H 'content-type: text/plain;' http://127.0.0.1:18443/wallet/benefactor

        // curl --user test --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "listunspent", "params": [1, 9999999, ["bcrt1phcnl4zcl2fu047pv4wx6y058v8u0n02at6lthvm7pcf2wrvjm5tqatn90k"] , true, { "minimumAmount": 0.005 } ]}' -H 'content-type: text/plain;' http://127.0.0.1:18443/wallet/benefactor
        let url = "http://127.0.0.1:18443/wallet/benefactor";
        let user = "test".to_string();
        let pass = "1234".to_string();
        let rpc = Client::new(&url, Auth::UserPass(user, pass)).unwrap();
        let address = "bcrt1phcnl4zcl2fu047pv4wx6y058v8u0n02at6lthvm7pcf2wrvjm5tqatn90k";

        let address = Address::from_str(&address)
            .unwrap()
            .require_network(Network::Regtest)
            .unwrap();
        let out = rpc.list_unspent(Some(1), None, Some(&[&address]), Some(true), None);
        println!("out: {:?}", out);
    }
}
