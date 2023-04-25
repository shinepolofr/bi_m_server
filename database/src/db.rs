use std::time::Duration;

use config::CFG;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use tokio::sync::OnceCell;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn db_conn() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(CFG.database.url.clone());
    opt.max_connections(1000)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(15))
        .idle_timeout(Duration::from_secs(60))
        .sqlx_logging(true);
    let db = Database::connect(opt).await.expect("open db failed");
    println!("{:?}", db);
    db
}
