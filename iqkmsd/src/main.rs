use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:27100".parse().unwrap();
    let keyring = signing::Keyring::new();
    let eth_service = ethereum::SignerService::new(keyring);

    println!("Listening on {}", addr);

    Server::builder()
        .add_service(ethereum::SignerServer::new(eth_service))
        .serve(addr)
        .await?;

    Ok(())
}
