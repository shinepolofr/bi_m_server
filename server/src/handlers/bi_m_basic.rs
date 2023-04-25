use crate::common::res::{self, Res};
use axum::{http::StatusCode, Json};
use database::BiMCardNumberCrud;
use sea_orm::{DbErr, EntityTrait};

pub async fn get_all_card_number() -> Res<Vec<BiMCardNumberCrud::BiMCardNumber>> {
    let r = BiMCardNumberCrud::get_all().await;
    Res {
        code: 200,
        data: r,
        msg: "succes".to_string(),
    }
}

pub async fn add_all_card_number(
    Json(card_numbers): Json<Vec<BiMCardNumberCrud::BiMCardNumber>>,
) -> Res<bool> {
    let r = BiMCardNumberCrud::insert_all(card_numbers).await;
    match r {
        true => Res {
            code: 200,
            data: true,
            msg: "success".to_string(),
        },
        false => Res {
            code: 500,
            data: false,
            msg: "false".to_string(),
        },
    }
}
