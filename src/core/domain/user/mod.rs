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
    fist_name: String,
    last_name: String,
    email: String,
    password: String,
    phone: String,
    status: UserStatus,
}

#[derive(Debug, PartialEq)]
pub struct UserError(String);

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for UserError {}

#[allow(dead_code)]
impl User {
    pub fn login(self: &User) -> Result<(), UserError> {
        match self.status {
            UserStatus::Active => Ok(()),
            UserStatus::Inactive => Err(UserError("User is not active".to_owned())),
        }
    }

    pub fn logout(self: &User) -> Result<(), UserError> {
        match self.status {
            UserStatus::Active => Ok(()),
            UserStatus::Inactive => Err(UserError("User is not active".to_owned())),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::core::domain::user::{User, UserError, UserStatus};
    use rstest::rstest;

    #[rstest]
    #[case(UserStatus::Active, Ok(()))]
    #[case(UserStatus::Inactive, Err(UserError("User is not active".to_owned())))]
    fn test_login(#[case] status: UserStatus, #[case] expected_result: Result<(), UserError>) {
        // GIVEN
        let user = User {
            id: 100,
            username: "jdoe".to_owned(),
            fist_name: "John".to_owned(),
            last_name: "Doe".to_owned(),
            email: "john.doe@example".to_owned(),
            password: "secure-password-42".to_owned(),
            phone: "012345693".to_owned(),
            status: status,
        };

        // WHEN
        // THEN
        assert_eq!(user.login(), expected_result);
    }

    #[rstest]
    #[case(UserStatus::Active, Ok(()))]
    #[case(UserStatus::Inactive, Err(UserError("User is not active".to_owned())))]
    fn test_logout(#[case] status: UserStatus, #[case] expected_result: Result<(), UserError>) {
        // GIVEN
        let user = User {
            id: 100,
            username: "jdoe".to_owned(),
            fist_name: "John".to_owned(),
            last_name: "Doe".to_owned(),
            email: "john.doe@example".to_owned(),
            password: "secure-password-42".to_owned(),
            phone: "012345693".to_owned(),
            status: status,
        };

        // WHEN
        // THEN
        assert_eq!(user.logout(), expected_result);
    }
}
