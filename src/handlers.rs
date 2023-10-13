use axum::{
    Form,
    extract::{State, Path},
    response::Redirect
};

use crate::repository::DynRepository;
use crate::templates;
use crate::utils;
use crate::structs;

fn prepare_index_template(error_msg: &str, short_id: &str) -> templates::IndexTemplate {
    templates::IndexTemplate{ error_msg: format!("{}", error_msg), short_id: short_id.to_string() }
}

pub async fn index_handler() -> templates::IndexTemplate {
    return prepare_index_template("", "");
}

pub async fn short_link_handler(State(repository): State<DynRepository>, Path(short_id): Path<String>) -> Redirect {
    let short_url_item = repository.find_short_link(short_id).await;
    match short_url_item {
        Ok(value) => {
            Redirect::to(value.url.as_str())
        },
        Err(_) => {
            Redirect::to("/")
        }
    }
}

pub async fn new_link_handler(State(repository): State<DynRepository>, Form(payload): Form<structs::NewShortUrlBody>) -> templates::IndexTemplate {
    let is_valid_url = utils::check_url(&payload.url);
    if !is_valid_url {
        return prepare_index_template("Not a valid url", "")
    }

    let short_id = utils::generate_short_id();

    repository.create_short_link(
        short_id.clone(),
        payload.url,
    ).await;

    prepare_index_template("", short_id.as_str())
}