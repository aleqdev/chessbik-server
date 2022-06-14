use actix_web::Responder;

pub async fn index_html() -> actix_web::Result<impl Responder> {
    Ok(actix_files::NamedFile::open_async("./static/index.html").await?)
}
