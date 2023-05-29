use std::net::TcpListener;

use mail_newsletter::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    run(listener)?.await
}
// #[cfg(test)]
// mod tests {
//     use actix_web::HttpRequest;

//     use crate::health_check;
//     #[test]
//     fn health_check_succeeds() {
//         let response = health_check().await;
//         // This requires changing the return type of `health_check`
//         // from `impl Responder` to `HttpResponse` to compile
//         // You also need to import it with `use actix_web::HttpResponse`! assert!(response.status().is_success())
//     }
// }
