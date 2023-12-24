use crate::{
    exceptions::QuantumixException,
    utils::{
        password::{hash_password, verify_password},
        sequence::generate_sequence,
    },
};
use entity::account::{
    ActiveModel as AccountActiveModel, Column as AccountColumn, Entity as Account,
    Model as AccountModel,
};
use oblivion::models::render::BaseResponse;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::json;

pub async fn register(
    username: &str,
    tuta_mail: &str,
    password: &str,
    nickname: &str,
    db: &DatabaseConnection,
) -> Result<AccountModel, QuantumixException> {
    let password = hash_password(password)?;

    let new_user_model = AccountActiveModel {
        sequence: sea_orm::ActiveValue::Set(generate_sequence(db).await.to_string()),
        username: sea_orm::ActiveValue::Set(username.to_string()),
        tuta_mail: sea_orm::ActiveValue::Set(tuta_mail.to_string()),
        password: sea_orm::ActiveValue::Set(password),
        nickname: sea_orm::ActiveValue::Set(nickname.to_string()),
        ..Default::default()
    };

    let new_user = new_user_model.insert(db).await.unwrap();

    Ok(new_user)
}

pub async fn login(identify: &str, password: &str, db: &DatabaseConnection) -> BaseResponse {
    let user_find = Account::find()
        .filter(
            AccountColumn::Username
                .eq(identify)
                .or(AccountColumn::TutaMail.eq(identify)),
        )
        .one(db)
        .await
        .unwrap();
    let user = if user_find.is_none() {
        return BaseResponse::JsonResponse(
            json!({"status": false, "msg": format!("用户[{}]不存在!", identify)}),
            404,
        );
    } else {
        user_find.unwrap()
    };

    if verify_password(password, &user.password) {
        BaseResponse::JsonResponse(json!({"status": true, "msg": "身份认证成功!"}), 200)
    } else {
        BaseResponse::JsonResponse(json!({"status": false, "msg": "密码错误!"}), 403)
    }
}
