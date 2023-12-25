use entity::account::Entity as Account;
use entity::project::{
    ActiveModel as ProjectActiveModel, Column as ProjectColumn, Entity as Project,
    Model as ProjectModel,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};

use crate::exceptions::QuantumixException;
use crate::views::project::{FilterProjectData, NewProjectData};

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
    let user_find = Account::find_by_id(leader_id).all(db).await.unwrap();
    if user_find.first().is_none() {
        return Err(QuantumixException::ColumnNotFound {
            table: "account".to_string(),
            field: "id".to_string(),
            data: leader_id.to_string(),
        });
    };
    let mut new_project_model: ProjectActiveModel = get_by_id(project_id, db).await?;
    new_project_model.leader_id = sea_orm::ActiveValue::Set(Some(leader_id));
    new_project_model.update(db).await.unwrap();
    Ok(true)
}

pub(crate) async fn finish_project(
    project_id: i32,
    db: &DatabaseConnection,
) -> Result<bool, QuantumixException> {
    let mut project: ProjectActiveModel = get_by_id(project_id, db).await?;
    project.is_checked = sea_orm::ActiveValue::Set(true);
    project.update(db).await.unwrap();
    Ok(true)
}

pub(crate) async fn filter_projects(data: FilterProjectData,
                                    db: &DatabaseConnection) -> Result<Vec<serde_json::Value>, QuantumixException> {
    println!("{:?}", data);
    let mut select_project = Project::find();
    if let Some(project_id) = data.project_id {
        select_project = select_project.filter(ProjectColumn::Id.eq(project_id));
    }
    if let Some(creator_id) = data.creator_id {
        select_project = select_project.filter(ProjectColumn::CreatorId.eq(creator_id));
    }
    if let Some(priority) = data.priority {
        select_project = select_project.filter(ProjectColumn::Priority.eq(priority));
    }
    if let Some(start_time) = data.start_time {
        select_project = select_project.filter(ProjectColumn::LeaderId.eq(start_time));
    }
    if let Some(name) = data.name{
        select_project = select_project.filter(ProjectColumn::Name.eq(name));
    }

    if let Some(is_checked) = data.is_checked{
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

async fn get_by_id(project_id: i32, db: &DatabaseConnection) -> Result<ProjectActiveModel, QuantumixException> {
    let result = Project::find_by_id(project_id)
        .one(db)
        .await
        .unwrap();
    let project: ProjectActiveModel =  if result.is_none() {
        return Err(QuantumixException::ColumnNotFound {
            table: "project".to_string(),
            field: "id".to_string(),
            data: project_id.to_string(),
        })
    } else {
        result.unwrap().into()
    };
    Ok(project)
}

async fn find_account_and_validate(id: i32, db: &DatabaseConnection) -> Result<(), QuantumixException> {
    let user_find = Account::find_by_id(id)
        .one(db)
        .await
        .unwrap();
    if user_find.is_none() {
        return Err(QuantumixException::ColumnNotFound {
            table: "account".to_string(),
            field: "id".to_string(),
            data: id.to_string(),
        });
    }
    Ok(())
}