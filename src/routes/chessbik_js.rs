use actix_web::Responder;

pub async fn chessbik_js() -> actix_web::Result<impl Responder> {
    Ok(actix_files::NamedFile::open_async("./static/chessbik.js").await?)
}
