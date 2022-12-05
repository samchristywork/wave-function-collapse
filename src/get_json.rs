use actix_web::{get, web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
    name_double: String,
    name_triple: String,
}

#[get("/json/{name}")]
async fn json(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
        name_double: name.to_string() + name.as_str(),
        name_triple: name.to_string() + name.as_str() + name.as_str(),
    };
    Ok(web::Json(obj))
}
