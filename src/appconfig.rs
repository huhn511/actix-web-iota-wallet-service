use actix_web::web;

use crate::handlers::{accounts};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    // domain includes: /accounts/{account_id}
    cfg.service(
        web::scope("/accounts")
            .service(
                web::resource("")
                    .route(web::get().to(accounts::get_accounts))
                    .route(web::post().to(accounts::add_account)),
            )
            .service(
                web::scope("/{account_id}")
                    .service(
                        web::resource("")
                            .route(web::get().to(accounts::get_account_detail))
                    )
            ),
    );
}