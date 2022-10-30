use askama::Template;

#[derive(Template)]
#[template(path = "sign_in.html")]
pub struct SignIn {
    pub error: bool,
}
