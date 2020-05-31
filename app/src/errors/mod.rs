use async_graphql::FieldError;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use serde_json::json;
use std::error::Error;
use std::fmt::Display;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum SrvError {
    InternalServerError,
    NotFound,
    Unauthorized(UnauthorizedInfo),
    Duplicate(DuplicateErrorInfo),
    ValidationError(ValidationErrors),
}

#[derive(Debug)]
pub struct DuplicateErrorInfo {
    pub origin: String,
    pub info: String,
}

#[derive(Debug, Serialize)]
pub struct UnauthorizedInfo {
    pub data: String,
}

impl From<SrvError> for FieldError {
    fn from(err: SrvError) -> Self {
        use SrvError::*;
        let (title, extensions) = match err {
            InternalServerError => ("InternalServerError", json!("")),
            Unauthorized(error_info) => ("Unauthorized", json!({ "info": error_info })),
            NotFound => ("NotFound", json!("")),
            Duplicate(error_info) => ("DUPLICATE", json!({ "info": error_info.origin })),
            ValidationError(errors) => {
                let errors = serde_json::to_value(&errors).unwrap_or(serde_json::Value::Null);
                (
                    "VALIDATION",
                    json!({"info": "Verify the Input data", "errors": errors }),
                )
            }
        };
        FieldError(title.to_string(), Some(extensions))
    }
}

impl From<DBError> for SrvError {
    fn from(error: DBError) -> SrvError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, _info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    return SrvError::Duplicate(DuplicateErrorInfo {
                        origin: _info.message().to_string(),
                        info: _info.details().unwrap_or("No Info").to_string(),
                    });
                }
                SrvError::InternalServerError
            }
            DBError::NotFound => SrvError::NotFound,
            e => {
                println!("{:?}", e);
                SrvError::InternalServerError
            }
        }
    }
}

impl From<validator::ValidationErrors> for SrvError {
    fn from(error: ValidationErrors) -> SrvError {
        SrvError::ValidationError(error)
    }
}
