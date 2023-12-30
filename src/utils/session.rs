use crate::settings::DATABASE_URL;
use chrono::{DateTime, Local};
use entity::account::Model as AccountModel;
use entity::prelude::Account;
use entity::prelude::Session;
use entity::session;
use oblivion::models::render::BaseResponse;
use oblivion::utils::parser::OblivionRequest;
use sea_orm::{ColumnTrait, Database, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use serde_json::json;

pub(crate) async fn get_account_and_db(
    req: &mut OblivionRequest,
) -> Result<(AccountModel, DatabaseConnection), BaseResponse> {
    let db = Database::connect(DATABASE_URL).await.map_err(|_| {
        BaseResponse::JsonResponse(json!({"status": false, "msg": "数据库连接失败！"}), 500)
    })?;

    let user_id = match get_uid(req, &db).await {
        Some(id) => id,
        None => {
            return Err(BaseResponse::JsonResponse(
                json!({"status": false, "msg": "令牌过期或不存在，请重新登录！"}),
                403,
            ));
        }
    };

    let account = match Account::find_by_id(user_id).one(&db).await.unwrap() {
        Some(account) => account,
        None => {
            return Err(BaseResponse::JsonResponse(
                json!({"status": false, "msg": "令牌过期或不存在，请重新登录！"}),
                403,
            ));
        }
    };

    Ok((account, db))
}

pub(crate) async fn get_uid(req: &mut OblivionRequest, db: &DatabaseConnection) -> Option<i32> {
    if let Some(model) = get_session(req, db).await {
        Some(model.user_id)
    } else {
        None
    }
}

pub(crate) async fn get_session(
    req: &mut OblivionRequest,
    db: &DatabaseConnection,
) -> Option<session::Model> {
    let data = req.get_post();

    match data["session_key"].as_str() {
        Some(session_key) => match validate_and_return_session(&db, session_key, true).await {
            Ok(model) => model,
            Err(_) => None,
        },
        None => None,
    }
}

pub(crate) async fn validate_and_handle_session(
    req: &mut OblivionRequest,
    db: &DatabaseConnection,
) -> Option<BaseResponse> {
    match get_session(req, db).await {
        Some(_) => None,
        None => Some(BaseResponse::JsonResponse(
            json!({"status": false, "msg": "令牌过期或不存在，请重新登录！"}),
            403,
        )),
    }
}

pub(crate) async fn validate_and_return_session(
    db: &DatabaseConnection,
    session_key: &str,
    delete_expired: bool,
) -> Result<Option<session::Model>, DbErr> {
    let session = match Session::find()
        .filter(session::Column::SessionKey.eq(session_key))
        .one(db)
        .await
        .unwrap()
    {
        Some(session) => session,
        None => {
            return Ok(None);
        }
    };
    let expire_time =
        DateTime::parse_from_str(&session.expire_time, "%Y-%m-%d %H:%M:%S%.f %:z").unwrap();
    if Local::now() > expire_time {
        if delete_expired {
            Session::delete_by_id(session.id).exec(db).await?;
        }
        return Ok(None);
    }
    Ok(Some(session))
}
