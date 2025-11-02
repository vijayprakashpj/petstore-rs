#[derive(Debug)]
pub enum UserStatus {
    LoggedIn = 1,
    LoggedOut = 0,
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
    user_status: UserStatus,
}

impl User {
    pub fn login() -> Result((), UserError) {
        todo!()
    }

    pub fn logout() -> Result((), UserError) {
        todo!()
    }
}
