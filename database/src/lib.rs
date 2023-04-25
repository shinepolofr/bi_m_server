mod db;
mod generate;
pub mod common;
pub use common::{bi_m_card_number_crud as BiMCardNumberCrud};

#[tokio::test]
async fn test_db_connc() {
    db_conn().await;
}
