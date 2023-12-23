use crate::{
    handlers::account::{login, register},
    settings::DATABASE_URL,
    utils::email::is_valid_email,
};
use entity::account::{Column as AccountColumn, Entity as Account};
use futures::future::{BoxFuture, FutureExt};
use oblivion::models::render::BaseResponse;
use oblivion::oblivion_codegen::async_route;
use oblivion::utils::parser::OblivionRequest;
use sea_orm::{ColumnTrait, Database, EntityTrait, QueryFilter};
use serde_json::json;

#[async_route]
async fn login_handler(mut req: OblivionRequest) -> BaseResponse {
    let db = Database::connect(DATABASE_URL).await.unwrap();

    let post = req.get_post();
    let identify = if !post["identify"].is_null() {
        post["identify"].as_str().unwrap().to_string()
    } else {
        return BaseResponse::JsonResponse(
            json!({"status": false, "msg": "未接收到用户名或Tuta邮箱!"}),
            403,
        );
    };
    let password = if !post["password"].is_null() {
        post["password"].as_str().unwrap().to_string()
    } else {
        return BaseResponse::JsonResponse(json!({"status": false, "msg": "未接收到密码!"}), 403);
    };
    login(&identify, &password, &db).await
}

#[async_route]
async fn register_handler(mut req: OblivionRequest) -> BaseResponse {
    let db = Database::connect(DATABASE_URL).await.unwrap();

    let post = req.get_post();
    let username = if !post["username"].is_null() {
        post["username"].as_str().unwrap().to_string()
    } else {
        return BaseResponse::JsonResponse(json!({"status": false, "msg": "未接收到用户名!"}), 403);
    };
    let tuta_mail = if !post["tuta_mail"].is_null() {
        post["tuta_mail"].as_str().unwrap().to_string()
    } else {
        return BaseResponse::JsonResponse(
            json!({"status": false, "msg": "未接收到Tuta邮箱!"}),
            403,
        );
    };
    if !is_valid_email(&tuta_mail) {
        return BaseResponse::JsonResponse(
            json!({"status": false, "msg": "邮箱不是合法的Tuta邮箱!"}),
            403,
        );
    }
    let password = if !post["password"].is_null() {
        post["password"].as_str().unwrap().to_string()
    } else {
        return BaseResponse::JsonResponse(json!({"status": false, "msg": "未接收到密码!"}), 403);
    };
    let nickname = if !post["nickname"].is_null() {
        post["nickname"].as_str().unwrap().to_string()
    } else {
        return BaseResponse::JsonResponse(json!({"status": false, "msg": "未接收到昵称!"}), 403);
    };

    let account_find = Account::find()
        .filter(AccountColumn::Username.eq(username.clone()))
        .filter(AccountColumn::TutaMail.eq(tuta_mail.clone()))
        .filter(AccountColumn::Nickname.eq(nickname.clone()))
        .one(&db)
        .await
        .unwrap();

    if !account_find.is_none() {
        return BaseResponse::JsonResponse(json!({"status": false, "msg": "存在冲突信息!"}), 403);
    };

    let account = register(&username, &tuta_mail, &password, &nickname, &db)
        .await
        .unwrap();

    BaseResponse::JsonResponse(
        json!({"status": true, "msg": format!("用户[{}]创建成功!", account.username)}),
        200,
    )
}
