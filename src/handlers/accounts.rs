use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

use iota_wallet::{
    account_manager::AccountManager, client::ClientOptionsBuilder, message::MessageType,
    signing::SignerType,
};


#[derive(Deserialize, Serialize)]
pub struct Account {
    id: Option<i64>,
    account_type: Option<String>,
    name: Option<String>,
}

pub async fn get_accounts(_query: web::Query<()>) -> Result<HttpResponse, Error> {
    println!("wallet-service::get_accounts");
    Ok(HttpResponse::Ok().finish())
}

pub async fn add_account(_new_account: web::Json<Account>) -> Result<HttpResponse, Error> {
    println!("wallet-service::add_account");
    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize, Serialize, Debug)]
struct AccountDetailsResponse {
    balance: u64,
    recieve_address: String,
}

pub async fn get_account_detail(id: web::Path<String>) -> Result<HttpResponse, Error> {
    println!("wallet-service::get_account_detail");

    let account_string = format!("{}", id);

    println!("account: {}", account_string);

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

    
    let account = manager.get_account(account_string).await.unwrap();


    // let synced = manager.sync_accounts().unwrap().execute().await.unwrap();
    // println!("Synchronized {} accounts", synced.len());


    // get the address we're going to use
    let address = account.read().await.addresses().first().unwrap().clone();

    println!("Address {}", address.address().to_bech32());

    let send_res = AccountDetailsResponse {
        balance: account.balance().await.available,
        recieve_address: address.address().to_bech32(),
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("{:?}", send_res)))
}

pub async fn remove_account(_id: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

#[cfg(test)]
mod tests {
    use crate::appconfig::config_app;
    use actix_service::Service;
    use actix_web::{
        http::{header, StatusCode},
        test, App,
    };

    #[actix_rt::test]
    async fn test_add_account() {
        let mut app = test::init_service(App::new().configure(config_app)).await;

        let payload = r#"{"id":12345,"account_type":"fancy","name":"test"}"#.as_bytes();

        let req = test::TestRequest::post()
            .uri("/accounts")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
