use std::env;

use actix::{Actor, Addr};
use actix_files::Files;
use actix_web::{
    middleware,
    web::{self, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws;
use data_server::DataServer;

pub(crate) mod data;
mod data_server;
mod engine_actor;
mod routes;
mod websocket;

async fn ws(
    req: HttpRequest,
    stream: web::Payload,
    srv: Data<Addr<DataServer>>,
) -> Result<HttpResponse, Error> {
    if let Some(addr) = req.connection_info().realip_remote_addr() {
        let resp = ws::start(websocket::Ws::new(srv.as_ref().clone(), addr), &req, stream);
        resp
    } else {
        Ok(HttpResponse::Ok().finish())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::dev::Service;
    use futures_util::FutureExt;

    let port = env::var("CHESSBICK_SERVER_PORT")
        .or(env::var("PORT"))
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("CHESSBICK_SERVER_PORT must be a number");

    let data = DataServer::start_default();

    HttpServer::new(move || {
        App::new()
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("Cross-Origin-Opener-Policy", "same-origin"))
                    .add(("Cross-Origin-Embedder-Policy", "require-corp")),
            )
            .wrap(middleware::Compress::default())
            .wrap_fn(|req, srv| {
                srv.call(req).map(|mut res| {
                    if let Ok(ref mut res) = res {
                        res.response_mut().head_mut().set_camel_case_headers(true);
                    }
                    res
                })
            })
            .app_data(Data::new(data.clone()))
            .route("/ws", web::get().to(ws))
            .route("/", web::get().to(routes::index_html))
            .route("/index.html", web::get().to(routes::index_html))
            .route("/chessbik.js", web::get().to(routes::chessbik_js))
            .route("/chessbik_bg.wasm", web::get().to(routes::chessbik_bg_wasm))
            .service(Files::new("/snippets", "./static/snippets"))
            .service(Files::new("/assets/", "./static/assets/"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
