use actix_files::NamedFile;
use actix_web::Result;
use std::path::PathBuf;

pub async fn index() -> Result<NamedFile> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}
