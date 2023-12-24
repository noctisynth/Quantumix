use entity::account::Entity as Account;
use entity::project::{
    ActiveModel as ProjectActiveModel, Column as ProjectColumn, Entity as Project,
    Model as ProjectModel,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::exceptions::QuantumixException;
use crate::views::project::ProjectData;

pub async fn new_project(
    creator_id: i32,
    data: ProjectData,
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

pub async fn take_project(
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
    let project = Project::find_by_id(project_id).one(db).await.unwrap();
    let mut new_project_model: ProjectActiveModel = if project.is_none() {
        return Err(QuantumixException::ColumnNotFound {
            table: "project".to_string(),
            field: "id".to_string(),
            data: project_id.to_string(),
        });
    } else {
        project.unwrap().into()
    };
    new_project_model.leader_id = sea_orm::ActiveValue::Set(Some(leader_id));
    new_project_model.update(db).await.unwrap();
    Ok(true)
}

pub async fn finish_project(
    project_id: i32,
    db: &DatabaseConnection,
) -> Result<bool, QuantumixException> {
    let project_find = Project::find_by_id(project_id).one(db).await.unwrap();
    let mut project: ProjectActiveModel = if project_find.is_none() {
        return Err(QuantumixException::ColumnNotFound {
            table: "project".to_string(),
            field: "id".to_string(),
            data: project_id.to_string(),
        });
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
        return Err(QuantumixException::ColumnNotFound {
            table: "project".to_string(),
            field: "id".to_string(),
            data: project_id.to_string(),
        });
    } else {
        project_find.unwrap()
    })
}

async fn find_account_and_validate(id: i32, db: &DatabaseConnection) -> Result<(), QuantumixException> {
    let user_find = Account::find_by_id(id).one(db).await.unwrap();
    if user_find.is_none() {
        return Err(QuantumixException::ColumnNotFound {
            table: "account".to_string(),
            field: "id".to_string(),
            data: id.to_string(),
        });
    }
    Ok(())
}

pub async fn filter_projects_by_leader_id(
    leader_id: i32,
    db: &DatabaseConnection,
) -> Result<Vec<serde_json::Value>, QuantumixException> {
    find_account_and_validate(leader_id, db).await?;
    Ok(Project::find()
        .filter(ProjectColumn::LeaderId.eq(leader_id))
        .into_json()
        .all(db)
        .await
        .unwrap())
}

pub async fn filter_projects_by_creator_id(
    creator_id: i32,
    db: &DatabaseConnection,
) -> Result<Vec<serde_json::Value>, QuantumixException> {
    find_account_and_validate(creator_id, db).await?;
    Ok(Project::find()
        .filter(ProjectColumn::CreatorId.eq(creator_id))
        .into_json()
        .all(db)
        .await
        .unwrap())
}

