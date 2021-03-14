pub mod appconfig;
pub mod handlers;

use iota_wallet::{
    account_manager::AccountManager, client::ClientOptionsBuilder, message::MessageType,
    signing::SignerType,
};

pub async fn init() {

    let mut manager = AccountManager::builder().finish().await.unwrap();
    manager.set_stronghold_password("password").await.unwrap();
    manager
        .store_mnemonic(SignerType::Stronghold, None)
        .await
        .unwrap();

    let node_url = "https://api.hornet-1.testnet.chrysalis2.com";

    // create an account
    let client_options = ClientOptionsBuilder::new()
        .with_node(node_url).unwrap()
        .with_local_pow(false)
        .build().unwrap();


    // let account = manager
    //     .create_account(client_options).unwrap()
    //     .alias(account_string)
    //     .initialise()
    //     .await.unwrap();

    
    let account = manager.get_account("alias").await.unwrap();


    let synced = manager.sync_accounts().unwrap().execute().await.unwrap();
    println!("Synchronized {} accounts", synced.len());

}