use sea_orm::ActiveValue::NotSet;
use sea_orm::prelude::DateTime;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::Set;

use crate::db;
use crate::generate::prelude::ServiceBiMCardNumber;
use crate::generate::service_bi_m_card_number;
use db::db_conn;
use db::DB;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BiMCardNumber {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub card_number: String,
    pub create_date: DateTime,
}

impl From<service_bi_m_card_number::Model> for BiMCardNumber {
    fn from(value: service_bi_m_card_number::Model) -> Self {
        Self {
            id: value.id,
            card_number: value.card_number.unwrap(),
            create_date: value.create_date.unwrap(),
        }
    }
}

impl IntoActiveModel<service_bi_m_card_number::ActiveModel> for BiMCardNumber {
    fn into_active_model(self) -> service_bi_m_card_number::ActiveModel {
        service_bi_m_card_number::ActiveModel {
            id: NotSet,
            card_number: Set(Some(self.card_number)),
            create_date: Set(Some(self.create_date)),
        }
    }
}

pub async fn get_all() -> Vec<BiMCardNumber> {
    let db = DB.get_or_init(db_conn).await;
    let all_card_number = ServiceBiMCardNumber::find().all(db).await;
    match all_card_number {
        Ok(v) => {
            let mut r = vec![];
            v.iter().for_each(|e| {
                r.push(BiMCardNumber::from(e.to_owned()));
            });
            r
        }
        Err(e) => {
            println!("{}", e);
            vec![]
        }
    }
}

pub async fn insert_all(card_numbers: Vec<BiMCardNumber>) -> bool {
    let db = DB.get_or_init(db_conn).await;
    let mut r = vec![];
    card_numbers.iter().for_each(|e| {
        r.push(BiMCardNumber::into_active_model(e.to_owned()));
    });
    match ServiceBiMCardNumber::insert_many(r).exec(db).await {
        Ok(_) => true,
        Err(e) => {
            println!("{}", e);
            false
        }
    }
}
