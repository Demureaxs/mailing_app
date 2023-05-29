use mail_newsletter::configuration::get_configuration;
use mail_newsletter::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    let port = listener.local_addr().unwrap().port();
    run(listener)?.await
}
