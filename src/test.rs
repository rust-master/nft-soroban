#![cfg(test)]

use super::testutils::{register_test_contract as register_contract, NFTContract};
use super::NFTContractClient;
use crate::NFTDetail;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn create_contract() -> (NFTContractClient<'static>, Env, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let id: Address = register_contract(&env);
    let nft_contract: NFTContract = NFTContract::new(&env, id.clone());

    let admin: Address = Address::random(&env);
    let name: String = String::from_slice(&env, "ART NFT");
    let symbol: String = String::from_slice(&env, "ANFT");

    let client: NFTContractClient<'_> = nft_contract.client();

    client.initialize(&admin, &name, &symbol);

    (client, env.clone(), id)
}

struct Setup {
    env: Env,
    client: NFTContractClient<'static>,
    contract_address: Address,
    minter: Address,
    token_uri: String,
    token_id: u128,
}

impl Setup {
    fn new() -> Self {
        let contract_client = create_contract();
        let client = contract_client.0;
        let env = contract_client.1;
        let contract_address = contract_client.2;

        // Mint NFT with minter account
        let minter: Address = Address::random(&env);
        let token_uri: String = String::from_slice(
            &env,
            "https://cdn.pixabay.com/photo/2017/09/12/11/56/universe-2742113_1280.jpg",
        );

        let token_id: u128 = client.mint(&minter, &token_uri);

        Self {
            env: env,
            client: client,
            contract_address: contract_address,
            minter: minter,
            token_uri: token_uri,
            token_id: token_id,
        }
    }
}

#[test]
fn test_mint() {
    let setup = Setup::new();

    let expected_nft: NFTDetail = NFTDetail {
        owner: setup.minter,
        uri: setup.token_uri,
    };

    let nft_detail: NFTDetail = setup.client.get_nft_detail(&setup.token_id);

    assert_eq!(expected_nft.owner, nft_detail.owner);
    assert_eq!(expected_nft.uri, nft_detail.uri);
}

#[test]
fn test_nft_transfer_from() {
    let setup = Setup::new();
    let receiver: Address = Address::random(&setup.env);

    setup
        .client
        .transfer_from(&setup.minter, &receiver, &setup.token_id);

    let nft_detail: NFTDetail = setup.client.get_nft_detail(&setup.token_id);

    assert_eq!(receiver, nft_detail.owner);
}

#[test]
fn test_burn() {
    let setup = Setup::new();

    setup.client.burn(&setup.minter, &setup.token_id);

    let nft_detail: NFTDetail = setup.client.get_nft_detail(&setup.token_id);

    let expected_uri: String = String::from_slice(&setup.env, "");

    assert_eq!(setup.contract_address, nft_detail.owner);
    assert_eq!(&expected_uri, &nft_detail.uri);
}
