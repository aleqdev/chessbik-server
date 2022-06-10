use std::env;

use actix_files::Files;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

mod data;
mod websocket;

async fn ws(
    req: HttpRequest,
    stream: web::Payload,
    data: data::DataTy,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(websocket::Ws { data }, &req, stream);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::dev::Service;
    use futures_util::FutureExt;

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let data = data::ChessbikData::new_actix();

    HttpServer::new(move || {
        App::new()
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("Cross-Origin-Opener-Policy", "same-origin"))
                    .add(("Cross-Origin-Embedder-Policy", "require-corp")),
            )
            .wrap_fn(|req, srv| {
                srv.call(req).map(|mut res| {
                    if let Ok(ref mut res) = res {
                        res.response_mut().head_mut().set_camel_case_headers(true);
                    }
                    res
                })
            })
            .app_data(data.clone())
            .route("/ws", web::get().to(ws))
            .service(Files::new("/", "./static/"))
            .service(Files::new("/assets/", "./static/assets/"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
