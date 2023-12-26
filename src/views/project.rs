use futures::future::{BoxFuture, FutureExt};
use oblivion::models::render::BaseResponse;
use oblivion::oblivion_codegen::async_route;
use oblivion::utils::parser::OblivionRequest;
use sea_orm::{Database, EntityTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::handlers::project::{filter_projects, finish_project, new_project, take_project};
use crate::settings::DATABASE_URL;
use crate::utils::permission::determine_permission;
use crate::utils::session::{get_account_and_db, validate_and_handle_session};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NewProjectData {
    pub(crate) name: String,
    pub(crate) leader_id: Option<i32>,
    pub(crate) priority: i32,
    pub(crate) content: String,
    pub(crate) permission: Option<i32>,
    pub(crate) start_time: Option<String>,
}

#[async_route]
pub(crate) async fn new_project_handler(mut req: OblivionRequest) -> BaseResponse {
    let (account, db) = match get_account_and_db(&mut req).await {
        Ok(result) => result,
        Err(response) => return response,
    };

    let data: NewProjectData = match serde_json::from_value(req.get_post()) {
        Ok(result) => match result {
            Some(data) => data,
            None => {
                return BaseResponse::JsonResponse(
                    json!({"status": false, "msg": "参数列表不匹配，请确认必填参数正确填入。"}),
                    403,
                );
            }
        },
        Err(_) => {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg": "参数列表不匹配，请确认必填参数正确填入。"}),
                403,
            );
        }
    };

    if let Some(permission) = data.permission {
        if account.permission > permission {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg":"项目创建失败！确认参数是否填写正确！" }),
                403,
            );
        }
    }

    match new_project(account.id, data, &db).await {
        Ok(result) => {
            let msg = format!("编号{}-{}创建成功！", result.id, result.name);
            BaseResponse::JsonResponse(json!({"status": true, "msg": msg }), 200)
        }
        Err(_) => BaseResponse::JsonResponse(
            json!({"status": false, "msg": "创建异常！请联系管理员！"}),
            403,
        ),
    }
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
            let msg = if result {
                "项目承接成功！"
            } else {
                "项目已被承接！"
            };
            return BaseResponse::JsonResponse(json!({"status": result, "msg": msg }), 200);
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
    pub(crate) is_checked: Option<bool>,
    pub(crate) permission: Option<i32>,
}

#[async_route]
pub(crate) async fn filter_projects_handler(mut req: OblivionRequest) -> BaseResponse {
    let (account, db) = match get_account_and_db(&mut req).await {
        Ok(result) => result,
        Err(response) => return response,
    };

    let data = match serde_json::from_value::<FilterProjectData>(req.get_post()) {
        Ok(data) => data,
        Err(_) => {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg":"查询失败！请确认参数是否填写正确！" }),
                403,
            );
        }
    };

    let permission = determine_permission(account.permission, data.permission);

    println!("FilterData = {:?}", data);
    return match filter_projects(data, permission, &db).await {
        Ok(result) => BaseResponse::JsonResponse(
            json!({"status": true, "msg": "查询成功！", "data": result }),
            200,
        ),
        Err(_) => BaseResponse::JsonResponse(json!({"status": false, "msg": "查询失败！"}), 500),
    };
}

#[async_route]
pub(crate) async fn finish_project_handler(mut req: OblivionRequest) -> BaseResponse {
    let (account, db) = match get_account_and_db(&mut req).await {
        Ok(result) => result,
        Err(response) => return response,
    };

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
    if let Ok(result) = finish_project(project_id as i32, account.permission, &db).await {
        let msg = if result {
            "项目结项成功！"
        } else {
            "项目已结项！"
        };
        return BaseResponse::JsonResponse(json!({"status": true, "msg": msg }), 200);
    }

    BaseResponse::JsonResponse(json!({"status": false, "msg": "项目结项失败！" }), 403)
}
