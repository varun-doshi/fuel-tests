// use fuels::{prelude::*, types::ContractId};

// // Load abi from json
// abigen!(Contract(
//     name = "MyContract",
//     abi = "out/debug/testy-abi.json"
// ));

// async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId) {
//     // Launch a local network and deploy the contract

//     //set utxo_validation=false. Now we can transact without a private key
//     let mut provider_config: fuels::test_helpers::NodeConfig = NodeConfig::default();
//     provider_config.utxo_validation = false;

//     let mut wallets = launch_custom_provider_and_get_wallets(
//         WalletsConfig::new(
//             Some(1),             /* Single wallet */
//             Some(1),             /* Single coin (UTXO) */
//             Some(1_000_000_000), /* Amount per coin */
//         ),
//         Some(provider_config),
//         None,
//     )
//     .await
//     .unwrap();
//     let wallet = wallets.pop().unwrap();

//     let id = Contract::load_from("./out/debug/testy.bin", LoadConfiguration::default())
//         .unwrap()
//         .deploy(&wallet, TxPolicies::default())
//         .await
//         .unwrap();

//     let instance = MyContract::new(id.clone(), wallet);

//     (instance, id.into())
// }

// #[tokio::test]
// async fn can_get_contract_id() {
//     let (_instance, _id) = get_contract_instance().await;

//     // Now you have an instance of your contract you can use to test each function
//     let value = _instance
//         .methods()
//         .test_number()
//         .call()
//         .await
//         .unwrap()
//         .value;
//     println!("{}", value);
// }
