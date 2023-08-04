use std::ptr::null;

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol,
};

const METADATA_KEY: Symbol = symbol_short!("METADATA");
const COUNTER: Symbol = symbol_short!("COUNTER");

#[derive(Clone)]
#[contracttype]
pub struct NFTMetadata {
    pub name: String,
    pub symbol: String,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
}

#[derive(Clone)]
#[contracttype]
pub struct NFTDetail {
    pub owner: Address,
    pub uri: String,
}

pub trait NFTTrait {
    fn initialize(env: Env, admin: Address, name: String, symbol: String);

    fn mint(env: Env, to: Address, token_uri: String);

    fn burn(env: Env, to: Address, token_id: u128);

    fn transfer_from(env: Env, from: Address, to: Address, token_id: u128);

    fn get_nft_detail(env: Env, token_id: u128) -> NFTDetail;

    fn read_administrator(env: Env) -> Address;

    fn has_administrator(env: Env) -> bool;

    fn has_nft_owner(env: Env, account: Address, token_id: u128) -> bool;

    fn name(env: Env) -> String;

    fn symbol(env: Env) -> String;
}

#[contract]
pub struct NFTContract;

#[contractimpl]
impl NFTTrait for NFTContract {
    fn initialize(env: Env, admin: Address, name: String, symbol: String) {
        if Self::has_administrator(env.clone()) {
            panic!("already initialized")
        }

        let metadata = NFTMetadata { name, symbol };

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&METADATA_KEY, &metadata);
    }

    fn mint(env: Env, to: Address, token_uri: String) {
        to.require_auth();

        if to == env.current_contract_address() {
            panic!("Sender can not be contract address")
        } else if token_uri == String::from_slice(&env, "") {
            panic!("Token URI can not be empty")
        }

        let mut token_id: u128 = env.storage().instance().get(&COUNTER).unwrap_or(1);

        token_id += 1;

        let nft_detail: NFTDetail = NFTDetail {
            owner: to,
            uri: token_uri,
        };

        env.storage().instance().set(&token_id, &nft_detail);
        env.storage().instance().set(&COUNTER, &token_id);
    }

    fn burn(env: Env, to: Address, token_id: u128) {
        to.require_auth();

        if Self::has_nft_owner(env.clone(), to.clone(), token_id) {
            panic!("Invalid Sender")
        } else if to == env.current_contract_address() {
            panic!("Sender can not be contract address")
        }

        let mut nft_detail = Self::get_nft_detail(env.clone(), token_id.clone());

        if nft_detail.owner != to || nft_detail.owner == env.current_contract_address() {
            panic!("NFT not exist")
        }

        nft_detail.owner = env.current_contract_address();
        nft_detail.uri = String::from_slice(&env, "");

        env.storage().instance().set(&token_id, &nft_detail);
    }

    fn transfer_from(env: Env, from: Address, to: Address, token_id: u128) {
        from.require_auth();

        if Self::has_nft_owner(env.clone(), from.clone(), token_id) {
            panic!("Invalid Sender")
        } else if from == env.current_contract_address() {
            panic!("Sender can not be contract address")
        }

        let mut nft_detail = Self::get_nft_detail(env.clone(), token_id.clone());

        if nft_detail.owner != to || nft_detail.owner == env.current_contract_address() {
            panic!("NFT not exist")
        }

        nft_detail.owner = to;

        env.storage().instance().set(&token_id, &nft_detail);
    }

    fn get_nft_detail(env: Env, token_id: u128) -> NFTDetail {
        let detail: NFTDetail = env
            .storage()
            .instance()
            .get(&token_id)
            .unwrap_or(NFTDetail {
                owner: env.current_contract_address(),
                uri: String::from_slice(&env, ""),
            });

        return detail;
    }

    fn read_administrator(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }

    fn has_administrator(env: Env) -> bool {
        let key = DataKey::Admin;
        env.storage().instance().has(&key)
    }

    fn has_nft_owner(env: Env, account: Address, token_id: u128) -> bool {
        let nft_detail = Self::get_nft_detail(env.clone(), token_id.clone());

        if nft_detail.owner != account {
            return true;
        } else {
            return false;
        }
    }

    fn name(env: Env) -> String {
        let metadata: NFTMetadata = env.storage().persistent().get(&METADATA_KEY).unwrap();

        metadata.name
    }

    fn symbol(env: Env) -> String {
        let metadata: NFTMetadata = env.storage().persistent().get(&METADATA_KEY).unwrap();

        metadata.symbol
    }
}
