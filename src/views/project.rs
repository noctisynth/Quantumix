use futures::future::{BoxFuture, FutureExt};
use oblivion::models::render::BaseResponse;
use oblivion::oblivion_codegen::async_route;
use oblivion::utils::parser::OblivionRequest;
use sea_orm::Database;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::handlers::project::new_project;
use crate::settings::DATABASE_URL;
use crate::utils::session::get_uid;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ProjectData {
    pub(crate) name: String,
    pub(crate) leader_id: Option<i32>,
    pub(crate) priority: i32,
    pub(crate) content: String,
    pub(crate) start_time: Option<String>,
}

#[async_route]
async fn create_project(mut req: OblivionRequest) -> BaseResponse {
    let db = Database::connect(DATABASE_URL).await.unwrap();
    let data = req.get_post();
    let user_id = match get_uid(&mut req, &db).await {
        Some(id) => { id }
        None => {
            return BaseResponse::JsonResponse(
                json!({"status": false, "msg": "令牌过期或不存在，请重新登录！"}),
                403,
            );
        }
    };
    return if let Ok(project_data) = serde_json::from_value::<ProjectData>(data) {
        match new_project(user_id,
                         project_data,
                          &db).await {
            Ok(result) => {
                let msg = format!("编号{}-{}创建成功！", result.id, result.name);
                BaseResponse::JsonResponse(
                    json!({"status": true, "msg": msg }),
                    200,
                )
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
