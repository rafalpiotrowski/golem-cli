use std::str::FromStr;
use std::{collections::HashMap, env};
use secp256k1::SecretKey;
use serde::Deserialize;

pub struct GolemToken {
    pub address: web3::types::Address,
    pub network: web3::Web3<web3::transports::Http>,
    pub accounts: HashMap<&'static str, Account>
}

#[derive(Debug, Copy, Clone)]
pub struct Account {
    pub address: web3::types::Address,
    pub pk: SecretKey
}


#[derive(Deserialize, Debug)]
pub struct Operation {
    pub from: web3::types::Address,
    pub to: web3::types::Address,
    pub method_name: String,
    pub value: usize,
}

impl GolemToken {
    pub fn new(network_url: &str) -> Self {
        GolemToken {
            address: {
                let value = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS is not set");
                value.parse::<web3::types::Address>().unwrap()
            },
            network : {
                let transport = web3::transports::Http::new(network_url).unwrap();
                web3::Web3::new(transport)
            },
            accounts: HashMap::new()
        }
    }

    pub async fn initialize_accounts(&mut self) -> web3::error::Result {
        let account1_address = env::var("ACCOUNT1_ADDRESS").expect("ACCOUNT1_ADDRESS is not set");
        let account1_pk = env::var("ACCOUNT1_PK").expect("ACCOUNT1_PK is not set");
        let account = Account {
            address: account1_address.parse::<web3::types::Address>().unwrap(),
            pk: SecretKey::from_str(account1_pk.as_str()).unwrap()
        };
        self.accounts.insert("A1", account);

        let account2_address = env::var("ACCOUNT2_ADDRESS").expect("ACCOUNT2_ADDRESS is not set");
        let account2_pk = env::var("ACCOUNT2_PK").expect("ACCOUNT2_PK is not set");
        let account = Account {
            address: account2_address.parse::<web3::types::Address>().unwrap(),
            pk: SecretKey::from_str(account2_pk.as_str()).unwrap()
        };
        self.accounts.insert("A2", account);

        let mut accounts = self.network.eth().accounts().await?;
        accounts.push(self.accounts["A1"].address);
        accounts.push(self.accounts["A2"].address);

        println!("Accounts: {:?}", accounts);
        
        Ok(())
    }

    pub async fn print_balances(&self) -> web3::error::Result {
        println!("Calling balance.");
        for account in &self.accounts {
            let balance = self.network.eth().balance(account.1.address, None).await?;
            println!("Balance of {:?}: {}", account.1.address, balance);
        }    
        Ok(())
    }
}