use env_logger::Env;

mod get_json;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));


    web::serve().await
}
