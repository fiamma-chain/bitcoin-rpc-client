use bitcoin::{Address, Network};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderParams {
    pub network: Network,
    pub http_endpoint: String,
}

impl ProviderParams {
    pub fn new(network: Network, http_endpoint: String) -> Self {
        Self {
            network,
            http_endpoint,
        }
    }

    pub fn local() -> Self {
        Self {
            network: Network::Regtest,
            http_endpoint: "http://127.0.0.1:43000".to_string(),
        }
    }

    pub fn dev() -> Self {
        Self {
            network: Network::Signet,
            http_endpoint: "http://127.0.0.1:53000".to_string(),
        }
    }

    pub fn signet() -> Self {
        Self {
            network: Network::Signet,
            http_endpoint: "http://127.0.0.1:53000".to_string(),
        }
    }

    pub fn dev_regtest() -> Self {
        Self {
            network: Network::Regtest,
            http_endpoint: "http://dev.fiammachain.io:43000".to_string(),
        }
    }

    pub fn is_dev(&self) -> bool {
        self.http_endpoint == "http://dev.fiammachain.io:43000"
    }

    pub fn bitcoin_url(&self) -> String {
        match self.network {
            Network::Regtest => {
                if self.is_dev() {
                    "".to_string()
                } else {
                    "http://127.0.0.1:18443".to_string()
                }
            }
            Network::Signet => "http://127.0.0.1:38332".to_string(),
            _ => panic!("other bitcoin network not supported"),
        }
    }

    pub fn bitcoin_username(&self) -> String {
        match self.network {
            Network::Regtest => "test".to_string(),
            Network::Signet => "fiamma".to_string(),
            _ => panic!("other bitcoin network not supported"),
        }
    }

    pub fn bitcoin_password(&self) -> String {
        match self.network {
            Network::Regtest => "1234".to_string(),
            Network::Signet => "fiamma".to_string(),
            _ => panic!("other bitcoin network not supported"),
        }
    }
}
