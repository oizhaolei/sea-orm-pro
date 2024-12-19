pub mod admin;
pub mod auth;
pub mod graphql;
pub mod user;

// Response of web controller
#[derive(Debug, serde::Serialize)]
pub struct Res {
    success: bool,
    message: String,
}

impl Res {
    // Success
    pub fn success<T: ToString>(message: T) -> Self {
        Self {
            success: true,
            message: message.to_string(),
        }
    }

    // Failed
    pub fn fail<T: ToString>(message: T) -> Self {
        Self {
            success: false,
            message: message.to_string(),
        }
    }
}
