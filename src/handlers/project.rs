use crate::exceptions::QuantumixException;
use crate::utils::account as account_utils;
use crate::views::project::{FilterProjectData, NewProjectData};
use entity::project::{
    ActiveModel as ProjectActiveModel, Column as ProjectColumn, Entity as Project,
    Model as ProjectModel,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
};

pub(crate) async fn new_project(
    creator_id: i32,
    data: NewProjectData,
    db: &DatabaseConnection,
) -> Result<ProjectModel, QuantumixException> {
    let new_project_model = ProjectActiveModel {
        name: sea_orm::ActiveValue::Set(data.name.to_string()),
        creator_id: sea_orm::ActiveValue::Set(Some(creator_id)),
        leader_id: sea_orm::ActiveValue::Set(data.leader_id),
        priority: sea_orm::ActiveValue::Set(data.priority),
        content: sea_orm::ActiveValue::Set(data.content),
        start_time: sea_orm::ActiveValue::Set(data.start_time),
        permission: sea_orm::ActiveValue::Set(data.permission),
        ..Default::default()
    };
    match new_project_model.insert(db).await {
        Ok(model) => Ok(model),
        Err(error) => Err(QuantumixException::CreateFieldFailed {
            name: data.name,
            error,
        }),
    }
}

pub(crate) async fn take_project(
    leader_id: i32,
    project_id: i32,
    db: &DatabaseConnection,
) -> Result<bool, QuantumixException> {
    let user = account_utils::find_account(leader_id, db).await?;
    let mut new_project_model: ProjectActiveModel = get_by_id(project_id, db).await?;
    if let Some(_) = new_project_model.leader_id.unwrap() {
        println!("已被承接");
        return Ok(false);
    }
    new_project_model.leader_id = sea_orm::ActiveValue::Set(Some(user.id));
    new_project_model.update(db).await.unwrap();
    Ok(true)
}

pub(crate) async fn finish_project(
    project_id: i32,
    permission: i32,
    db: &DatabaseConnection,
) -> Result<bool, QuantumixException> {
    let mut project: ProjectActiveModel = get_by_id(project_id, db).await?;

    let is_checked = project.is_checked.unwrap();
    if is_checked {
        return Ok(false);
    }
    if let Some(perm) = project.permission.as_ref() {
        if permission > *perm {
            return Ok(false);
        }
    }
    project.is_checked = sea_orm::ActiveValue::Set(true);
    project.update(db).await.unwrap();
    Ok(true)
}

pub(crate) async fn filter_projects(
    data: FilterProjectData,
    permission: i32,
    db: &DatabaseConnection,
) -> Result<Vec<serde_json::Value>, QuantumixException> {
    let mut select_project = Project::find();

    select_project = select_project.filter(
        ProjectColumn::Permission
            .gte(permission)
            .or(ProjectColumn::Permission.is_null()),
    );

    if let Some(project_id) = data.project_id {
        select_project = select_project.filter(ProjectColumn::Id.eq(project_id));
    }
    if let Some(creator_id) = data.creator_id {
        select_project = select_project.filter(ProjectColumn::CreatorId.eq(creator_id));
    }
    if let Some(priority) = data.priority {
        select_project = select_project.filter(ProjectColumn::Priority.eq(priority));
    }
    if let Some(name) = data.name {
        select_project = select_project.filter(ProjectColumn::Name.eq(name));
    }

    if let Some(is_checked) = data.is_checked {
        select_project = select_project.filter(ProjectColumn::IsChecked.eq(is_checked));
    }

    let projects = select_project
        .into_json()
        .paginate(db, data.size)
        .fetch_page(data.page - 1)
        .await
        .unwrap();

    Ok(projects)
}

async fn get_by_id(
    project_id: i32,
    db: &DatabaseConnection,
) -> Result<ProjectActiveModel, QuantumixException> {
    let result = Project::find_by_id(project_id).one(db).await.unwrap();
    let project: ProjectActiveModel = if result.is_none() {
        return Err(QuantumixException::ColumnNotFound {
            table: "project".to_string(),
            field: "id".to_string(),
            data: project_id.to_string(),
        });
    } else {
        result.unwrap().into()
    };
    Ok(project)
}
