use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

mod get_json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let host = "0.0.0.0";
    let port = 8080;
    log::info!("Serving on {} port {}", host, port);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(get_json::json)
            .service(Files::new("/images", "static/images/").show_files_listing())
            .service(Files::new("/", "./static/root/").index_file("index.html"))
    })
    .bind((host, port))?
    .run()
    .await
}
