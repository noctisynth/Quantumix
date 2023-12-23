use entity::account::Entity as Account;
use entity::project::{
    ActiveModel as ProjectActiveModel, Column as ProjectColumn, Entity as Project,
    Model as ProjectModel,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::exceptions::QuantumixException;

pub async fn new_project(
    name: &str,
    user_id: Option<i32>,
    priority: i32,
    content: &str,
    start_time: Option<&str>,
    db: &DatabaseConnection,
) -> Result<ProjectModel, QuantumixException> {
    let new_project_model = ProjectActiveModel {
        name: sea_orm::ActiveValue::Set(name.to_string()),
        user_id: sea_orm::ActiveValue::Set(user_id),
        priority: sea_orm::ActiveValue::Set(priority),
        content: sea_orm::ActiveValue::Set(content.to_string()),
        start_time: sea_orm::ActiveValue::Set(Some(start_time.unwrap().to_string())),
        ..Default::default()
    };
    match new_project_model.insert(db).await {
        Ok(model) => Ok(model),
        Err(error) => Err(QuantumixException::CreateFieldFailed(Some(format!(
            "创建项目[{}]时出现异常: {}",
            name, error
        )))),
    }
}

pub async fn take_project(
    user_id: i32,
    project_id: i32,
    db: &DatabaseConnection,
) -> Result<bool, QuantumixException> {
    let user_find = Account::find_by_id(user_id).all(db).await.unwrap();
    if user_find.first().is_none() {
        return Err(QuantumixException::ColumnNotFound(
            format!("未能在数据库中查找到序列为[{}]的用户", user_id).into(),
        ));
    };
    let project = Project::find_by_id(project_id).one(db).await.unwrap();
    let mut new_project_model: ProjectActiveModel = if project.is_none() {
        return Err(QuantumixException::ColumnNotFound(
            format!("未能在数据库中查找到序列为[{}]的工程项目", user_id).into(),
        ));
    } else {
        project.unwrap().into()
    };
    new_project_model.user_id = sea_orm::ActiveValue::Set(Some(user_id));
    new_project_model.update(db).await.unwrap();
    Ok(true)
}

pub async fn finish_project(
    project_id: i32,
    db: &DatabaseConnection,
) -> Result<bool, QuantumixException> {
    let project_find = Project::find_by_id(project_id).one(db).await.unwrap();
    let mut project: ProjectActiveModel = if project_find.is_none() {
        return Err(QuantumixException::ColumnNotFound(
            format!("未能在数据库中查找到序列为[{}]的工程项目", project_id).into(),
        ));
    } else {
        project_find.unwrap().into()
    };
    project.is_checked = sea_orm::ActiveValue::Set(true);
    project.update(db).await.unwrap();
    Ok(true)
}

pub async fn get_project(
    project_id: i32,
    db: &DatabaseConnection,
) -> Result<serde_json::Value, QuantumixException> {
    let project_find: Option<serde_json::Value> = Project::find_by_id(project_id)
        .into_json()
        .one(db)
        .await
        .unwrap();
    Ok(if project_find.is_none() {
        return Err(QuantumixException::ColumnNotFound(
            format!("未能在数据库中查找到序列为[{}]的工程项目", project_id).into(),
        ));
    } else {
        project_find.unwrap()
    })
}

pub async fn filter_projects(
    user_id: i32,
    db: &DatabaseConnection,
) -> Result<Vec<serde_json::Value>, QuantumixException> {
    let user_find = Account::find_by_id(user_id).one(db).await.unwrap();
    if user_find.is_none() {
        return Err(QuantumixException::ColumnNotFound(
            format!("用户序列[{}]不存在", user_id).into(),
        ));
    }
    Ok(Project::find()
        .filter(ProjectColumn::UserId.eq(user_id))
        .into_json()
        .all(db)
        .await
        .unwrap())
}
