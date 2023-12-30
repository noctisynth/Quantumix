use crate::exceptions::QuantumixException;
use entity::account::{Entity as Account, Model as AccountModel};
use sea_orm::{DatabaseConnection, EntityTrait};

pub(crate) async fn find_account(
    id: i32,
    db: &DatabaseConnection,
) -> Result<AccountModel, QuantumixException> {
    match Account::find_by_id(id).one(db).await.unwrap() {
        Some(user_find) => Ok(user_find),
        None => Err(QuantumixException::ColumnNotFound {
            table: "account".to_string(),
            field: "id".to_string(),
            data: id.to_string(),
        }),
    }
}
