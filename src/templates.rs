use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub(crate) error_msg: String,
    pub(crate) short_id: String,
}