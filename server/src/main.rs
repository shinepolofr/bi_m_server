mod common;
mod handlers;

use axum::routing::{get, post};
use axum::Router;
use handlers::bi_m_basic::{add_all_card_number, get_all_card_number};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/cardNumbers", get(get_all_card_number))
        .route("/cardNumbers", post(add_all_card_number));
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test() -> &'static str {
    "Hello World!"
}
