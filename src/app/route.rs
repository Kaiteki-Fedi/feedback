use crate::app::api::feedback;
use actix_web::web;

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    cfg.service((feedback::list, feedback::create))
}
