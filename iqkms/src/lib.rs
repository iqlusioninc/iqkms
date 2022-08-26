//! iqkms client library

pub use proto;

#[cfg(test)]
mod tests {
    use super::proto::ethereum::{tx_signer_client::TxSignerClient, SignTxRequest};
    use tonic::Request;

    #[tokio::test]
    async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = TxSignerClient::connect("http://[::1]:50051").await?;

        // TODO(tarcieri): transaction body
        let tx_body = vec![];

        // TODO(tarcieri): real signing key address
        let address = "0x27b1fdb04752bbc536007a920d24acb045561c26".to_owned();

        let request = Request::new(SignTxRequest { tx_body, address });
        let response = client.sign_tx(request).await?;

        println!("RESPONSE={:?}", response);
        Ok(())
    }
}
