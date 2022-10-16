use askama::Template;
use axum::response::{Html, IntoResponse};

pub fn from_template<T>(template: T) -> impl IntoResponse 
where
    T: Template,
{
    Html(template.render().unwrap()).into_response()
}
