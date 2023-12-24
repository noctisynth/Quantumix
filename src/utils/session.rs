use chrono::{DateTime, Local};
use entity::prelude::Session;
use entity::session;
use oblivion::utils::parser::OblivionRequest;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

pub async fn get_uid(mut req: &mut OblivionRequest, db: &DatabaseConnection) -> Option<i32> {
    let data = req.get_post();
    let session_key = match data["session_key"].as_str() {
        Some(result) => result,
        None => {
            return None;
        }
    };
    match validate_and_return_session(&db, session_key, true).await {
        Ok(session) => {
            return match session {
                Some(model) => { Some(model.id) }
                None => { None }
            };
        }
        Err(_err) => { None }
    }
}

pub async fn validate_session(db: &DatabaseConnection,
                              session_key: &str,
                              delete_expired: bool) -> Result<bool, DbErr> {
    match validate_and_return_session(db, session_key, delete_expired).await {
        Ok(session) => {
            if session == None { Ok(false) } else { Ok(true) }
        }
        Err(err) => { Err(err) }
    }
}

pub async fn validate_and_return_session(db: &DatabaseConnection,
                                         session_key: &str,
                                         delete_expired: bool) -> Result<Option<session::Model>, DbErr> {
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
        if delete_expired { Session::delete_by_id(session.id).exec(db).await?; }
        return Ok(None);
    }
    Ok(Some(session))
}