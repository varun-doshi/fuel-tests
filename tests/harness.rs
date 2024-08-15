use fuels::{prelude::*, types::ContractId};

// Load abi from json


// abigen!(Contract(
//     name = "MyContract",
//     abi = "out/release/test-v2-abi.json"
// ));


async fn get_contract_instance()  {

    abigen!(Contract(
        name = "MyContract",
        abi = "out/release/test-v2-abi.json"
    ));

    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await
    .unwrap();
    let wallet = wallets.pop().unwrap();

    let id = Contract::load_from(
        "./out/debug/test-v2.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default())
    .await
    .unwrap();

    // let instance = MyContract::new(id.clone(), wallet);

    // (instance, id.into())
}

#[tokio::test]
async fn can_get_contract_id() {
     get_contract_instance().await;
    // let (_instance, _id) = get_contract_instance().await;

    
}
