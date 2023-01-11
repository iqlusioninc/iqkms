use signing::SigningService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:27100".parse().unwrap();

    let signing_service = tower::ServiceBuilder::new()
        .buffer(10) // TODO(tarcieri): tune buffer size
        .service(SigningService::new());

    let eth_service = ethereum::SignerService::new(signing_service);

    // TODO(tarcieri): use tracing for logging
    println!("Listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(ethereum::SignerServer::new(eth_service))
        .serve(addr)
        .await?;

    Ok(())
}
