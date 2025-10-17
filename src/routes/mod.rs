use actix_web::web;
use actix_web_static_files::ResourceFiles;

pub mod generate_upload;
pub mod get_upload;
pub mod give_upload;
pub mod upload;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
mod generated_static_files {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

pub fn register_all(cfg: &mut web::ServiceConfig) {
    cfg.service(ResourceFiles::new(
        "/assets",
        generated_static_files::generate(),
    ));
    cfg.service(generate_upload::generate_upload);
    cfg.service(upload::upload);
    cfg.service(get_upload::get_upload);
    cfg.service(give_upload::give_upload);
}
