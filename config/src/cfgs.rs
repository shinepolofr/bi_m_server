use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configs {
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}
