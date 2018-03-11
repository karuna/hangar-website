#[derive(Serialize)]
pub struct Signin {}
#[derive(Serialize)]
pub struct Signup {
    pub email: Option<String>,
}
