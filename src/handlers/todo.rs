use crate::exceptions::QuantumixException;
use crate::utils::account as account_utils;
use crate::views::todo::{FilterTodoData, NewTodoData};
use entity::todo::{
    ActiveModel as TodoActiveModel, Column as TodoColumn, Entity as Todo, Model as TodoModel,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
};

pub(crate) async fn new_todo(
    creator_id: i32,
    data: NewTodoData,
    db: &DatabaseConnection,
) -> Result<TodoModel, QuantumixException> {
    let new_todo_model = TodoActiveModel {
        name: sea_orm::ActiveValue::Set(data.name.to_string()),
        priority: sea_orm::ActiveValue::Set(data.priority),
        content: sea_orm::ActiveValue::Set(data.content),
        description: sea_orm::ActiveValue::Set(data.description),
        project_id: sea_orm::ActiveValue::Set(data.project_id),
        creator_id: sea_orm::ActiveValue::Set(Some(creator_id)),
        owner_id: sea_orm::ActiveValue::Set(data.owner_id),
        startline: sea_orm::ActiveValue::Set(data.start_line),
        endline: sea_orm::ActiveValue::Set(data.end_line),
        permission: sea_orm::ActiveValue::Set(data.permission),
        ..Default::default()
    };
    match new_todo_model.insert(db).await {
        Ok(model) => Ok(model),
        Err(error) => Err(QuantumixException::CreateFieldFailed {
            name: data.name,
            error,
        }),
    }
}

pub(crate) async fn finish_todo(
    todo_id: i32,
    permission: i32,
    db: &DatabaseConnection,
) -> Result<bool, QuantumixException> {
    let mut todo: TodoActiveModel = get_by_id(todo_id, db).await?;
    let is_checked = todo.is_checked.unwrap();
    if is_checked {
        return Ok(false);
    }

    if let Some(perm) = todo.permission.as_ref() {
        if permission > *perm {
            return Ok(false);
        }
    }
    todo.is_checked = sea_orm::ActiveValue::Set(true);
    todo.update(db).await.unwrap();
    Ok(true)
}

pub(crate) async fn take_todo(
    owner_id: i32,
    todo_id: i32,
    db: &DatabaseConnection,
) -> Result<bool, QuantumixException> {
    let user = account_utils::find_account(owner_id, db).await?;
    let mut new_todo_model: TodoActiveModel = get_by_id(todo_id, db).await?;
    if let Some(_) = new_todo_model.owner_id.unwrap() {
        return Ok(false);
    }
    new_todo_model.owner_id = sea_orm::ActiveValue::Set(Some(user.id));
    new_todo_model.update(db).await.unwrap();
    Ok(true)
}

pub(crate) async fn filter_todos(
    data: FilterTodoData,
    permission: i32,
    db: &DatabaseConnection,
) -> Result<Vec<serde_json::Value>, QuantumixException> {
    let mut select_todo = Todo::find();

    select_todo = select_todo.filter(
        TodoColumn::Permission
            .gte(permission)
            .or(TodoColumn::Permission.is_null()),
    );

    if let Some(todo_id) = data.todo_id {
        select_todo = select_todo.filter(TodoColumn::Id.eq(todo_id));
    }

    if let Some(creator_id) = data.creator_id {
        select_todo = select_todo.filter(TodoColumn::CreatorId.eq(creator_id));
    }
    if let Some(name) = data.name {
        select_todo = select_todo.filter(TodoColumn::Name.eq(name));
    }

    if let Some(priority) = data.priority {
        select_todo = select_todo.filter(TodoColumn::Priority.eq(priority));
    }

    if let Some(content) = data.content {
        select_todo = select_todo.filter(TodoColumn::Content.eq(content));
    }

    if let Some(description) = data.description {
        select_todo = select_todo.filter(TodoColumn::Description.eq(description));
    }

    if let Some(project_id) = data.project_id {
        select_todo = select_todo.filter(TodoColumn::ProjectId.eq(project_id));
    }

    if let Some(owner_id) = data.owner_id {
        select_todo = select_todo.filter(TodoColumn::OwnerId.eq(owner_id));
    }

    if let Some(start_line) = data.start_line {
        select_todo = select_todo.filter(TodoColumn::Startline.eq(start_line));
    }

    if let Some(end_line) = data.end_line {
        select_todo = select_todo.filter(TodoColumn::Endline.eq(end_line));
    }

    if let Some(parent_id) = data.parent_id {
        select_todo = select_todo.filter(TodoColumn::Parent.eq(parent_id));
    }

    if let Some(is_checked) = data.is_checked {
        select_todo = select_todo.filter(TodoColumn::IsChecked.eq(is_checked));
    }

    let projects = select_todo
        .into_json()
        .paginate(db, data.size)
        .fetch_page(data.page - 1)
        .await
        .unwrap();

    Ok(projects)
}

async fn get_by_id(
    todo_id: i32,
    db: &DatabaseConnection,
) -> Result<TodoActiveModel, QuantumixException> {
    let result = Todo::find_by_id(todo_id).one(db).await.unwrap();
    let project: TodoActiveModel = if result.is_none() {
        return Err(QuantumixException::ColumnNotFound {
            table: "todo".to_string(),
            field: "id".to_string(),
            data: todo_id.to_string(),
        });
    } else {
        result.unwrap().into()
    };
    Ok(project)
}
