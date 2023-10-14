use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PebbleQueryError {
    #[error("Invalid condition: {0}")]
    InvalidConditionOperator(String),
    #[error("Invalid operator: {0}")]
    InvalidOperator(String),
    #[error("Invalid field: {0}")]
    InvalidField(String),
    #[error("Missing value: {0}")]
    MissingValue(String),
    #[error("SeaOrmDbError: {0}")]
    SeaOrmDbError(#[from] DbErr),
}
