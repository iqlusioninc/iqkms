use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let keyring = signing::KeyRing::new();
    let eth_service = ethereum::RpcService::new(keyring);

    println!("Listening on {}", addr);

    Server::builder()
        .add_service(ethereum::TxSignerServer::new(eth_service))
        .serve(addr)
        .await?;

    Ok(())
}
