use std::str::FromStr;
use std::sync::Arc;
use std::{collections::HashMap, env};
use secp256k1::SecretKey;
use serde::Deserialize;

pub struct GolemToken {
    pub address: web3::types::Address,
    pub network: web3::Web3<web3::transports::Http>,
    pub accounts: HashMap<&'static str, Account>,
    pub contract: Option<web3::contract::Contract<web3::transports::Http>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Account {
    pub address: web3::types::Address,
    pub pk: SecretKey
}


#[derive(Deserialize, Debug, Clone)]
pub struct Operation {
    pub method_name: String,
    pub from: Option<web3::types::Address>,
    pub to: Option<web3::types::Address>,
    pub num_tokens: Option<u64>,
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
            accounts: HashMap::new(),
            contract: None
        }
    }

    pub async fn initialize(&mut self) -> web3::error::Result {
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

        println!("Accounts: {:?}", self.accounts);

        self.contract = Some(web3::contract::Contract::from_json(
            self.network.eth(),
            self.address,
            include_bytes!("../src/contract/golem-token-interface.json"),
        ).unwrap());

        println!("contract address: {:?}", self.address);
        
        Ok(())
    }

    pub async fn print_balances(&self) -> web3::error::Result {
        //println!("Calling balance.");
        for account in &self.accounts {
            let balance = self.network.eth().balance(account.1.address, None).await?;
            println!("Balance of {:?}: {}", account.1.address, balance);
        }    
        Ok(())
    }

    pub async fn execute(&self, operation: Operation) -> web3::error::Result {
        // let mut rng = rand::rngs::OsRng;
        // let x = rand::Rng::gen_range(&mut rng, 0..10);
        //  println!("exeuting: {:?} - will continue in {}", operation, x);
        // // std::thread::sleep(std::time::Duration::from_secs(x));
        // tokio::time::sleep(tokio::time::Duration::from_secs(x)).await;

        match operation.method_name.as_str() {
            "totalSupply" => self.total_supply().await.unwrap(),
            "balanceOf" => self.balance_of(&operation).await.unwrap(),
            "transfer" => self.transfer(&operation).await.unwrap(),
            _ => println!("function {} not supported", operation.method_name)
        }

        //println!("exeuting: {:?} -- COMPLETED", operation);
        Ok(())
    }

    async fn total_supply(&self) -> web3::error::Result {
        let c = Arc::new(self.contract.to_owned().unwrap());
        let result = c.query("totalSupply", (), None, web3::contract::Options::default(), None);
        let storage: u64 = result.await.unwrap();
        println!("totalSupply: {}", storage);
        Ok(())
    }

    async fn balance_of(&self, operation: &Operation) -> web3::error::Result {
        let c = Arc::new(self.contract.to_owned().unwrap());
        let result = c.query(
            "balanceOf", 
            (operation.from.unwrap(),), 
            None, 
            web3::contract::Options::default(), 
            None);
        let storage: u64 = result.await.unwrap();
        println!("balanceOf: {:?} = {}", operation.from, storage);
        Ok(())
    }

    async fn transfer(&self, operation: &Operation) -> web3::error::Result {
        let c = Arc::new(self.contract.to_owned().unwrap());

        match self.accounts.values().into_iter().filter(|&a| a.address.eq(&operation.from.unwrap())).next() {
            None => println!("no accound"),
            Some(a) => {
                let prvk = a.pk;

                c.signed_call_with_confirmations(
                    "transfer",
                    (operation.to.unwrap(), operation.num_tokens.unwrap(), ), 
                    web3::contract::Options::default(), 
                    2, 
                    &prvk).await.unwrap();        
            }
        };

        Ok(())
    }
}