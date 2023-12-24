use sea_orm_migration::prelude::*;
use async_trait::async_trait;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("ALTER TABLE project ADD COLUMN creator_id INTEGER REFERENCES account(id);", ).await?;
        db.execute_unprepared("ALTER TABLE project RENAME COLUMN user_id TO leader_id;", ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("ALTER TABLE project DROP COLUMN creator_id;", ).await?;
        db.execute_unprepared("ALTER TABLE project RENAME COLUMN leader_id TO user_id;", ).await?;
        Ok(())
    }
}