use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
    // HttpResponse::Ok().json("Hey dickhead")
}
