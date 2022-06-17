use actix_web::{http::header, Responder};

pub async fn chessbik_bg_wasm() -> actix_web::Result<impl Responder> {
    Ok(
        actix_files::NamedFile::open_async("./static/chessbik_bg.wasm.gz")
            .await?
            .customize()
            .append_header(header::ContentEncoding::Gzip)
            .insert_header(header::ContentType("application/wasm".parse().unwrap())),
    )
}
