use crate::{
    handlers::account::{login, register},
    settings::DATABASE_URL,
    utils::email::EMAIL_VALIDATOR, // 已替换先前右键验证函数
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
    let identity = match post["identity"].as_str() {
        Some(result) => result,
        None => {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg": "未接收到用户名或Tuta邮箱!"}),
                403,
            );
        }
    };
    let password = match post["password"].as_str() {
        Some(result) => result,
        None => {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg": "未接收到密码!"}),
                403,
            )
        }
    };
    let unique_id = match post["unique_id"].as_str() {
        Some(result) => result,
        None => {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg": "未接收到设备标识!"}),
                403,
            )
        }
    };

    BaseResponse::JsonResponse(
        json!({"status": true, "msg": "身份验证成功", "session_key": match login(&identity, &password, &unique_id, &db).await {
            Ok(session_key) => session_key,
            Err(error) => {
                return BaseResponse::JsonResponse(json!({"status": false, "msg": error.to_string()}), 403);
            },
        }}),
        403,
    )
}

#[async_route]
async fn register_handler(mut req: OblivionRequest) -> BaseResponse {
    let db = Database::connect(DATABASE_URL).await.unwrap();

    let post = req.get_post();
    let username = match post["username"].as_str() {
        Some(result) => result,
        None => {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg": "未接收到用户名!"}),
                403,
            );
        }
    };
    let tuta_mail = match post["tuta_mail"].as_str() {
        Some(result) => {
            if !EMAIL_VALIDATOR.validate(&result) {
                return BaseResponse::JsonResponse(
                    json!({"status": false, "msg": "邮箱不是合法的Tuta邮箱!"}),
                    403,
                );
            };
            result
        }
        None => {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg": "未接收到Tuta邮箱!"}),
                403,
            );
        }
    };
    let password = match post["password"].as_str() {
        Some(result) => result,
        None => {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg": "未接收到密码!"}),
                403,
            );
        }
    };
    let nickname = match post["nickname"].as_str() {
        Some(result) => result,
        None => {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg": "未接收到昵称!"}),
                403,
            );
        }
    };

    let account_find = Account::find()
        .filter(AccountColumn::Username.eq(username))
        .filter(AccountColumn::TutaMail.eq(tuta_mail))
        .filter(AccountColumn::Nickname.eq(nickname))
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
