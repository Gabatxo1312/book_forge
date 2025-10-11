use std::{io};

use sea_orm::DbErr;

#[derive(Debug)]
pub enum AppError {
    ConfigError,
    ConfigLoadingError
}
