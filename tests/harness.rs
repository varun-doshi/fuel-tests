use std::str::FromStr;

use fuels::{
    accounts::wallet::Wallet,
    prelude::*,
    types::{bech32, ContractId, Identity},
};

// Load abi from json
abigen!(Contract(
    name = "TestContract",
    abi = "out/debug/testy-abi.json"
));

async fn get_contract_instance() -> (TestContract<WalletUnlocked>, ContractId) {
    // Launch a local network and deploy the contract

    //set utxo_validation=false. Now we can transact without a private key
    let mut provider_config: fuels::test_helpers::NodeConfig = NodeConfig::default();
    provider_config.utxo_validation = false;
    provider_config.block_production = Trigger::Instant;

    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        Some(provider_config),
        None,
    )
    .await
    .unwrap();
    let wallet = wallets.pop().unwrap();

    let id = Contract::load_from("./out/debug/testy.bin", LoadConfiguration::default())
        .unwrap()
        .deploy(&wallet, TxPolicies::default())
        .await
        .unwrap();

    let instance = TestContract::new(id.clone(), wallet);

    (instance, id.into())
}

async fn transfer_asset(user1: Wallet, user2: Wallet) {
    // let transfer_amount = 1;
    // let asset_id = Default::default();
    // let (_tx_id, _receipts) = user1
    //     .transfer(
    //         user2.address(),
    //         transfer_amount,
    //         asset_id,
    //         TxPolicies::default(),
    //     )
    //     .await?;
}

#[tokio::test]
async fn can_get_contract_id() {
    let provider = Provider::connect("testnet.fuel.network").await.unwrap();
    println!("{:?}", provider.node_info().await.unwrap());
    let address1 =
        Address::from_str("0x9c934dd1343708ee6294dba141dd3da620c84a6bc92e5b40850931807d3332c3")
            .unwrap();
    let address2 =
        Address::from_str("0xd56187fcff046dd76b11298a460e61438642e94bc40a1ad3d3c20caba45f288f")
            .unwrap();
    let iden = Identity::from(address1);
    let bech32_address1 = Bech32Address::from(address1);
    let bech32_address2 = Bech32Address::from(address2);

    let user1 = Wallet::from_address(bech32_address1, Some(provider.clone()));
    let user2 = Wallet::from_address(bech32_address2, Some(provider.clone()));

    println!("balance:{:?}", user1.get_balances().await.unwrap());

    transfer_asset(user1, user2).await;

    let (_instance, _id) = get_contract_instance().await;
    println!(
        "Block:{:?}",
        _instance
            .account()
            .try_provider()
            .unwrap()
            .latest_block_height()
            .await
    );

    // Now you have an instance of your contract you can use to test each function
    let value = _instance
        .methods()
        .get_counter()
        .call()
        .await
        .unwrap()
        .value;
    println!("{}", value);

    let initialization = _instance
        .methods()
        .initialize_counter(2)
        .call()
        .await
        .unwrap();

    println!(
        "Block:{:?}",
        _instance
            .account()
            .try_provider()
            .unwrap()
            .latest_block_height()
            .await
    );
    let value = _instance
        .methods()
        .get_counter()
        .call()
        .await
        .unwrap()
        .value;
    println!("{}", value);

    _instance
        .account()
        .try_provider()
        .unwrap()
        .produce_blocks(5, None)
        .await
        .unwrap();

    println!(
        "Block:{:?}",
        _instance
            .account()
            .try_provider()
            .unwrap()
            .latest_block_height()
            .await
    );
}

//0x9c934dd1343708ee6294dba141dd3da620c84a6bc92e5b40850931807d3332c3
