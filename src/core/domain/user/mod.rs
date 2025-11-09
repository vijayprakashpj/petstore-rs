use std::{error::Error, fmt};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum UserStatus {
    Active = 1,
    Inactive = 0,
}

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    id: i64,
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    phone: String,
    status: UserStatus,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct UserError(String);

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for UserError {}
