use crate::handlers::todo::{filter_todos, finish_todo, new_todo, take_todo};
use crate::settings::DATABASE_URL;
use crate::utils::permission::determine_permission;
use crate::utils::session::{get_account_and_db, validate_and_handle_session};
use futures::future::{BoxFuture, FutureExt};
use oblivion::models::render::BaseResponse;
use oblivion::oblivion_codegen::async_route;
use oblivion::utils::parser::OblivionRequest;
use sea_orm::Database;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NewTodoData {
    pub(crate) name: String,
    pub(crate) priority: i32,
    pub(crate) content: String,
    pub(crate) description: String,
    pub(crate) project_id: Option<i32>,
    pub(crate) owner_id: Option<i32>,
    pub(crate) permission: Option<i32>,
    pub(crate) start_line: Option<String>,
    pub(crate) end_line: Option<String>,
    pub(crate) parent_id: Option<i32>,
}

#[async_route]
pub(crate) async fn new_todo_handler(mut req: OblivionRequest) -> BaseResponse {
    let (account, db) = match get_account_and_db(&mut req).await {
        Ok(result) => result,
        Err(response) => return response,
    };

    let new_todo_data: NewTodoData = match serde_json::from_value(req.get_post()) {
        Ok(result) => match result {
            Some(data) => data,
            None => {
                return BaseResponse::JsonResponse(
                    json!({"status": false, "msg": "参数列表不匹配，请确认必填参数正确填入！"}),
                    403,
                );
            }
        },
        Err(_) => {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg": "参数列表不匹配，请确认必填参数正确填入！"}),
                403,
            );
        }
    };

    if let Some(permission) = new_todo_data.permission {
        if account.permission > permission {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg":"Todo创建失败，请确认参数是否填写正确！" }),
                403,
            );
        }
    }

    match new_todo(account.id, new_todo_data, &db).await {
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
pub(crate) struct TakeTodoData {
    pub(crate) owner_id: i32,
    pub(crate) todo_id: i32,
}

#[async_route]
pub(crate) async fn take_todo_handler(mut req: OblivionRequest) -> BaseResponse {
    let db = Database::connect(DATABASE_URL).await.unwrap();
    if let Some(response) = validate_and_handle_session(&mut req, &db).await {
        return response;
    }

    if let Ok(data) = serde_json::from_value::<TakeTodoData>(req.get_post()) {
        if let Ok(result) = take_todo(data.owner_id, data.todo_id, &db).await {
            let msg = if result {
                "Todo承接成功！"
            } else {
                "Todo已被承接！"
            };
            return BaseResponse::JsonResponse(json!({"status": result, "msg": msg }), 200);
        }
    };
    BaseResponse::JsonResponse(json!({"status": false, "msg": "Todo承接失败！" }), 403)
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct FilterTodoData {
    pub(crate) page: u64,
    pub(crate) size: u64,
    pub(crate) todo_id: Option<i32>,
    pub(crate) creator_id: Option<i32>,
    pub(crate) name: Option<String>,
    pub(crate) priority: Option<i32>,
    pub(crate) content: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) project_id: Option<i32>,
    pub(crate) owner_id: Option<i32>,
    pub(crate) permission: Option<i32>,
    pub(crate) start_line: Option<String>,
    pub(crate) end_line: Option<String>,
    pub(crate) parent_id: Option<i32>,
    pub(crate) is_checked: Option<bool>,
}

#[async_route]
pub(crate) async fn filter_todos_handler(mut req: OblivionRequest) -> BaseResponse {
    let (account, db) = match get_account_and_db(&mut req).await {
        Ok(result) => result,
        Err(response) => return response,
    };

    let data = match serde_json::from_value::<FilterTodoData>(req.get_post()) {
        Ok(data) => data,
        Err(_) => {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg":"查询失败！请确认参数是否填写正确！" }),
                403,
            );
        }
    };

    let permission = determine_permission(account.permission, data.permission);

    return match filter_todos(data, permission, &db).await {
        Ok(result) => BaseResponse::JsonResponse(
            json!({"status": true, "msg": "查询成功！", "data": result }),
            200,
        ),
        Err(_) => BaseResponse::JsonResponse(json!({"status": false, "msg": "查询失败！"}), 500),
    };
}

#[async_route]
pub(crate) async fn finish_todo_handler(mut req: OblivionRequest) -> BaseResponse {
    let (account, db) = match get_account_and_db(&mut req).await {
        Ok(result) => result,
        Err(response) => return response,
    };

    let post = req.get_post();
    let todo_id = match post["todo_id"].as_i64() {
        Some(result) => result,
        None => {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg": "未接收到待办编号!"}),
                403,
            );
        }
    };

    if let Ok(result) = finish_todo(todo_id as i32, account.permission, &db).await {
        let msg = if result {
            "待办完成！"
        } else {
            "待办已完成！"
        };
        return BaseResponse::JsonResponse(json!({"status": true, "msg": msg }), 200);
    }

    BaseResponse::JsonResponse(json!({"status": false, "msg": "待办操作失败！" }), 403)
}
