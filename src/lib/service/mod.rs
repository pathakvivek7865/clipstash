pub mod ask;
pub mod action; 

use crate::{ClipError, DataError};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("clip error: {0}")]
    Clip(#[from] ClipError),
    #[error("database error: {0}")]
    Data(#[from] DataError),
    #[error("not found")]
    NotFound,
    #[error("permission not met: {0}")]
    PermissionError(String),
}

// impl From<DataError> for ServiceError {
//     fn from(value: DataError) -> Self {
//         match value {
//             DataError::Database(d) => match d {
//                 sqlx::Error::RowNotFound => Self::NotFound,
//                 other => Self::Data(DataError::Database(other)),
//             },
//         }
//     }
// }

impl From<sqlx::Error> for ServiceError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound,
            other => Self::Data(DataError::Database(other)),
        }
    }
}
