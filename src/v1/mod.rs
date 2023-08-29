mod redeploy;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/v1")
        .service(redeploy::post_redeploy)
}