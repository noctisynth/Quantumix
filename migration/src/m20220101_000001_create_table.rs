use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

fn created_at(table_name: &str) -> String {
    format!(
        "CREATE TRIGGER IF NOT EXISTS {0}_updated_at
        AFTER UPDATE ON {0}
        FOR EACH ROW
        BEGIN
            UPDATE {0}
            SET updated_at = (datetime('now','localtime'))
            WHERE id = NEW.id;
        END;",
        table_name
    )
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permission::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Permission::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Permission::Level)
                            .unsigned()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Permission::Description)
                            .string()
                            .string_len(128)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Permission::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT (datetime('now','localtime'))")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Permission::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT (datetime('now','localtime'))")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Account::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Account::Sequence)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Account::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Account::TutaMail)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Account::Password).string().not_null())
                    .col(ColumnDef::new(Account::Nickname).string().not_null())
                    .col(
                        ColumnDef::new(Account::Sex)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Account::Country)
                            .string()
                            .string_len(16)
                            .not_null()
                            .default("中国"),
                    )
                    .col(ColumnDef::new(Account::FavoritesIcon).string().null())
                    .col(
                        ColumnDef::new(Account::Permission)
                            .integer()
                            .not_null()
                            .default(5),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Account::Table, Account::Permission)
                            .to(Permission::Table, Permission::Level),
                    )
                    .col(
                        ColumnDef::new(Account::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT (datetime('now','localtime'))")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Account::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT (datetime('now','localtime'))")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Session::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Session::SessionKey)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Session::UserID).integer().not_null())
                    .col(ColumnDef::new(Session::UniqueId).string().not_null())
                    .col(ColumnDef::new(Session::ExpireTime).timestamp().not_null())
                    .col(
                        ColumnDef::new(Session::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT (datetime('now','localtime'))")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Project::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Project::Name).string().not_null())
                    .col(ColumnDef::new(Project::UserID).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Project::Table, Project::UserID)
                            .to(Account::Table, Account::Id),
                    )
                    .col(ColumnDef::new(Project::Permission).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Project::Table, Project::Permission)
                            .to(Permission::Table, Permission::Level),
                    )
                    .col(ColumnDef::new(Project::Priority).integer().not_null())
                    .col(ColumnDef::new(Project::Content).string().not_null())
                    .col(ColumnDef::new(Project::StartTime).date_time().null())
                    .col(ColumnDef::new(Project::EndTime).date_time().null())
                    .col(
                        ColumnDef::new(Project::IsChecked)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Project::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT (datetime('now','localtime'))")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Project::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT (datetime('now','localtime'))")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Todo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Todo::ProjectID).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Todo::Table, Todo::ProjectID)
                            .to(Project::Table, Project::Id),
                    )
                    .col(ColumnDef::new(Todo::Name).string().not_null())
                    .col(ColumnDef::new(Todo::UserID).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Todo::Table, Todo::UserID)
                            .to(Account::Table, Account::Id),
                    )
                    .col(ColumnDef::new(Todo::Permission).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Todo::Table, Todo::Permission)
                            .to(Permission::Table, Permission::Level),
                    )
                    .col(ColumnDef::new(Todo::Priority).integer().not_null())
                    .col(ColumnDef::new(Todo::Content).string().not_null())
                    .col(ColumnDef::new(Todo::Description).string().not_null())
                    .col(ColumnDef::new(Todo::Startline).date_time().null())
                    .col(ColumnDef::new(Todo::Endline).date_time().null())
                    .col(ColumnDef::new(Todo::Parent).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("child")
                            .from(Todo::Table, Todo::Parent)
                            .to(Todo::Table, Todo::Id),
                    )
                    .col(
                        ColumnDef::new(Todo::IsChecked)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Todo::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT (datetime('now','localtime'))")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Todo::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT (datetime('now','localtime'))")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Task::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Task::ProjectID).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Task::Table, Task::ProjectID)
                            .to(Project::Table, Project::Id),
                    )
                    .col(ColumnDef::new(Task::Name).string().not_null())
                    .col(ColumnDef::new(Task::UserID).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Task::Table, Task::UserID)
                            .to(Account::Table, Account::Id),
                    )
                    .col(ColumnDef::new(Task::Permission).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Task::Table, Task::Permission)
                            .to(Permission::Table, Permission::Level),
                    )
                    .col(ColumnDef::new(Task::Priority).integer().not_null())
                    .col(ColumnDef::new(Task::Content).string().not_null())
                    .col(ColumnDef::new(Task::Description).string().not_null())
                    .col(ColumnDef::new(Task::Startline).date_time().null())
                    .col(ColumnDef::new(Task::Endline).date_time().null())
                    .col(ColumnDef::new(Task::Parent).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("child")
                            .from(Task::Table, Task::Parent)
                            .to(Task::Table, Task::Id),
                    )
                    .col(
                        ColumnDef::new(Task::IsChecked)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Task::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT (datetime('now','localtime'))")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Task::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT (datetime('now','localtime'))")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        db.execute_unprepared(&created_at("permission")).await?;
        db.execute_unprepared(&created_at("account")).await?;
        db.execute_unprepared(&created_at("project")).await?;
        db.execute_unprepared(&created_at("todo")).await?;
        db.execute_unprepared(&created_at("task")).await?;

        db.execute_unprepared(
            "INSERT OR IGNORE INTO permission (name, level, description) VALUES ('N5 权限', 0, '第五议会至高权限');",
        )
        .await?;
        db.execute_unprepared(
            "INSERT OR IGNORE INTO permission (name, level, description) VALUES ('4 级权限', 1, '');",
        )
        .await?;
        db.execute_unprepared(
            "INSERT OR IGNORE INTO permission (name, level, description) VALUES ('3 级权限', 2, '');",
        )
        .await?;
        db.execute_unprepared(
            "INSERT OR IGNORE INTO permission (name, level, description) VALUES ('2 级权限', 3, '');",
        )
        .await?;
        db.execute_unprepared(
            "INSERT OR IGNORE INTO permission (name, level, description) VALUES ('1 级信任', 4, '');",
        )
        .await?;
        db.execute_unprepared(
            "INSERT OR IGNORE INTO permission (name, level, description) VALUES ('0 级信任', 5, '');",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Permission {
    Table,
    Id,
    Name,
    Level,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Account {
    Table,
    Id,
    Sequence,
    Username,
    TutaMail,
    Password,
    Nickname,
    Sex,
    Country,
    FavoritesIcon,
    Permission,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Session {
    Table,
    Id,
    SessionKey,
    UserID,
    UniqueId,
    ExpireTime,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Project {
    Table,
    Id,
    Name,
    UserID,
    Permission,
    Priority,
    Content,
    StartTime,
    EndTime,
    IsChecked,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Todo {
    Table,
    Id,
    ProjectID,
    UserID,
    Name,
    Permission,
    Priority,
    Content,
    Description,
    Startline,
    Endline,
    Parent,
    IsChecked,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
    ProjectID,
    UserID,
    Name,
    Permission,
    Priority,
    Content,
    Description,
    Startline,
    Endline,
    Parent,
    IsChecked,
    CreatedAt,
    UpdatedAt,
}
