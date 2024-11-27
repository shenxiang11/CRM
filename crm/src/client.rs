use anyhow::Result;
use crm::pb::{crm_client::CrmClient, RecallRequest, RecallRequestBuilder, RemindRequest, WelcomeRequestBuilder};
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let pem = include_str!("../../fixtures/rootCA.pem");
    let tls = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(pem))
        .domain_name("localhost");
    let channel = Channel::from_static("https://[::1]:50000")
        .tls_config(tls)?
        .connect()
        .await?;

    let token = include_str!("../../fixtures/token").trim();
    let token: MetadataValue<_> = format!("Bearer {}", token).parse()?;

    let mut client = CrmClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    // Welcome
    let req = WelcomeRequestBuilder::default()
        .id(Uuid::new_v4().to_string())
        .interval(93u32)
        .content_ids([1u32, 2, 3])
        .build()?;

    let response = client.welcome(Request::new(req)).await?.into_inner();
    println!("Welcome Response: {:?}", response);

    // Recall
    let req = RecallRequest {
        id: Uuid::new_v4().to_string(),
        last_visit_interval: 93,
        content_ids: vec![1, 2, 3],
    };
    let response = client.recall(Request::new(req)).await?.into_inner();
    println!("Recall Response: {:?}", response);

    // Remind
    let req = RemindRequest {
        id: Uuid::new_v4().to_string(),
        last_visit_interval: 93,
    };
    let response = client.remind(Request::new(req)).await?.into_inner();
    println!("Remind Response: {:?}", response);

    Ok(())
}
