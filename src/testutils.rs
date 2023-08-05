#![cfg(test)]

use crate::NFTContractClient;

use soroban_sdk::{Address, Env};


pub fn register_test_contract(e: &Env) -> Address {
    e.register_contract(None, crate::NFTContract {})
}

pub struct NFTContract {
    env: Env,
    contract_id: Address,
}

impl NFTContract {
    #[must_use]
    pub fn client(&self) -> NFTContractClient<'static> {
        NFTContractClient::new(&self.env, &self.contract_id)
    }

    #[must_use]
    pub fn new(env: &Env, contract_id: Address) -> Self {
        Self {
            env: env.clone(),
            contract_id,
        }
    }
}
