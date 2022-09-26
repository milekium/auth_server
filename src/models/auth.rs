use validator::Validate;

#[derive(Validate, Debug)]
pub struct Credentials {
    pub username: String,
    #[validate(length(min = 3))]
    pub password: String,
}
