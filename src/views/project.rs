use futures::future::{BoxFuture, FutureExt};
use oblivion::models::render::BaseResponse;
use oblivion::oblivion_codegen::async_route;
use oblivion::utils::parser::OblivionRequest;
use sea_orm::Database;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::handlers::project::{filter_projects, finish_project, new_project, take_project};
use crate::settings::DATABASE_URL;
use crate::utils::session::{get_uid, validate_and_handle_session};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NewProjectData {
    pub(crate) name: String,
    pub(crate) leader_id: Option<i32>,
    pub(crate) priority: i32,
    pub(crate) content: String,
    pub(crate) start_time: Option<String>,
}


#[async_route]
pub(crate) async fn new_project_handler(mut req: OblivionRequest) -> BaseResponse {
    let db = Database::connect(DATABASE_URL).await.unwrap();
    let user_id = match get_uid(&mut req, &db).await {
        Some(id) => { id }
        None => { return BaseResponse::JsonResponse(json!({"status": false, "msg": "令牌过期或不存在，请重新登录！"}), 403); }
    };
    return if let Ok(data) = serde_json::from_value::<NewProjectData>(req.get_post()) {
        match new_project(user_id, data, &db).await {
            Ok(result) => {
                let msg = format!("编号{}-{}创建成功！", result.id, result.name);
                BaseResponse::JsonResponse(json!({"status": true, "msg": msg }), 200)
            }
            Err(_err) => {
                BaseResponse::JsonResponse(
                    json!({"status": false, "msg": "创建异常！请联系管理员！"}),
                    403,
                )
            }
        }
    } else {
        BaseResponse::JsonResponse(
            json!({"status": false, "msg": "参数列表不匹配，请确认必填参数正确填入。"}),
            403,
        )
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TakeProjectData {
    pub(crate) leader_id: i32,
    pub(crate) project_id: i32,
}

#[async_route]
pub(crate) async fn take_project_handler(mut req: OblivionRequest) -> BaseResponse {
    let db = Database::connect(DATABASE_URL).await.unwrap();
    if let Some(response) = validate_and_handle_session(&mut req, &db).await {
        return response;
    }
    if let Ok(data) = serde_json::from_value::<TakeProjectData>(req.get_post()) {
        if let Ok(result) = take_project(data.leader_id, data.project_id, &db).await {
            let msg = if result { "项目承接成功！" } else { "项目承接失败！" };
            let code = if result { 200 } else { 403 };
            return BaseResponse::JsonResponse(json!({"status": result, "msg": msg }), code);
        }
    };
    BaseResponse::JsonResponse(json!({"status": false, "msg": "项目承接失败！" }), 403)
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct FilterProjectData {
    pub(crate) page: u64,
    pub(crate) size: u64,
    pub(crate) project_id: Option<i32>,
    pub(crate) name: Option<String>,
    pub(crate) creator_id: Option<i32>,
    pub(crate) leader_id: Option<i32>,
    pub(crate) priority: Option<i32>,
    pub(crate) start_time: Option<String>,
    pub(crate) is_checked: Option<bool>
}

#[async_route]
pub(crate) async fn filter_project_handler(mut req: OblivionRequest) -> BaseResponse {
    let db = Database::connect(DATABASE_URL).await.unwrap();
    if let Some(response) = validate_and_handle_session(&mut req, &db).await {
        return response;
    }

    if let Ok(data) = serde_json::from_value::<FilterProjectData>(req.get_post()) {
        if let Ok(result) = filter_projects(data, &db).await {
            return BaseResponse::JsonResponse(json!({"status": true, "msg": "查询成功！", "data": result }), 200);
        }
    }

    BaseResponse::JsonResponse(json!({"status": false, "msg": "项目查询失败！" }), 403)
}

#[async_route]
pub(crate) async fn finish_project_handler(mut req: OblivionRequest) -> BaseResponse {
    let db = Database::connect(DATABASE_URL).await.unwrap();
    if let Some(response) = validate_and_handle_session(&mut req, &db).await {
        return response;
    }
    let post = req.get_post();
    let project_id = match post["project_id"].as_i64() {
        Some(result) => result,
        None => {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg": "未接收到项目编号!"}),
                403,
            );
        }
    };
    if let Ok(result) = finish_project(project_id as i32, &db).await {
        if result {
            return BaseResponse::JsonResponse(json!({"status": true, "msg": "项目结项成功！" }), 200);
        }
    }

    BaseResponse::JsonResponse(json!({"status": false, "msg": "项目结项失败！" }), 403)
}