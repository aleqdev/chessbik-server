use std::env;

use actix::{Actor, Addr, Recipient};
use actix_files::Files;
use actix_web::{
    middleware,
    web::{self, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws;
use chessbik_commons::WsMessage;
use data_server::DataServer;
use websocket::InternalWsMessage;

pub(crate) mod data;
mod data_server;
mod websocket;

pub fn send_to_recip(message: WsMessage, recip: &Recipient<InternalWsMessage>) {
    match serde_json::to_string(&message) {
        Ok(str) => {
            recip.do_send(InternalWsMessage(str));
        }
        Err(err) => {
            println!("error: failed to serialize message:\n{}", err);
        }
    }
}

async fn ws(
    req: HttpRequest,
    stream: web::Payload,
    srv: Data<Addr<DataServer>>,
) -> Result<HttpResponse, Error> {
    println!("{:?}", req.connection_info());
    if let Some(addr) = req.connection_info().peer_addr() {
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

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let data = DataServer::start_default();

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
            .app_data(Data::new(data.clone()))
            .route("/ws", web::get().to(ws))
            .service(Files::new("/", "./static/"))
            .service(Files::new("/assets/", "./static/assets/"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
