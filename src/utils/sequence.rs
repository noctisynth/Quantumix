use entity::account::{Column as AccountColumn, Entity as Account};
use rand::{rngs::OsRng, Rng};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn generate_sequence(db: &DatabaseConnection) -> i32 {
    let mut rng = OsRng;

    loop {
        let random_number: i32 = rng.gen_range(1000..9999);
        let account_find = Account::find()
            .filter(AccountColumn::Sequence.eq(random_number))
            .one(db)
            .await
            .unwrap();
        if account_find.is_none() {
            return random_number;
        }
    }
}
