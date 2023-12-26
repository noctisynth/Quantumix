use crate::exceptions::QuantumixException;
use entity::account::{Entity as Account, Model as AccountModel};
use sea_orm::{DatabaseConnection, EntityTrait};

pub(crate) async fn find_account(
    id: i32,
    db: &DatabaseConnection,
) -> Result<AccountModel, QuantumixException> {
    let user_find = Account::find_by_id(id).one(db).await.unwrap();
    if user_find.is_none() {
        return Err(QuantumixException::ColumnNotFound {
            table: "account".to_string(),
            field: "id".to_string(),
            data: id.to_string(),
        });
    }

    return if let Some(model) = user_find {
        Ok(model)
    } else {
        Err(QuantumixException::ColumnNotFound {
            table: "account".to_string(),
            field: "id".to_string(),
            data: id.to_string(),
        })
    };
}
