use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuantumixException {
    #[error("未能在表[{name}]中查找到序列为[{id}]的数据")]
    ColumnNotFound { id: i32, name: String },
    #[error("创建项目[{name}]时出现异常: {error:?}")]
    CreateFieldFailed { name: String, error: DbErr },
    #[error("密码加密时出现异常: {error:?}")]
    PasswordHashFailed { error: argon2::password_hash::Error },
    #[error("包含[{feature}]特征的数据已经存在")]
    ColumnExists { feature: String },
}
