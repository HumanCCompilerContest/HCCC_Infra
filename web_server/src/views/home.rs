use askama::Template;
use crate::views::partial::Submit;

#[derive(Template)]
#[template(path = "home.html")]
pub struct Home {
    pub submits: Vec<Submit>,
}
