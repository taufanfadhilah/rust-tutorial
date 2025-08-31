use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserReq {
    pub name: String,
}
