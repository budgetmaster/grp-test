use payments::{BtcPaymentRequest, bitcoin_client::BitcoinClient};
use tonic::{metadata::MetadataValue, transport::Channel, Request};

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;

    let token: MetadataValue<_> = "Bearer random2".parse()?;

    let mut client = BitcoinClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let request = tonic::Request::new(BtcPaymentRequest {
        from_addr: "123456".to_owned(),
        to_addr: "654321".to_owned(),
        amount: 22,
    });

    let response = client.send_payment(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
