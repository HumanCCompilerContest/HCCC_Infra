use askama::Template;

#[derive(Template)]
#[template(path = "sign_up.html")]
pub struct SignUp;
