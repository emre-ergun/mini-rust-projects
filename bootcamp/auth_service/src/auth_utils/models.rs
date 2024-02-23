#[derive(Debug)]
pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    pub fn new(username: String, password: String) -> Credentials {
        Credentials {
            username,
            password,
        }
    }
}