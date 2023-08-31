// This is not a core part of the package, 
// but is instead a helper main that allows 
// manual testing against a Postmark account
use std::env;

use postmark::reqwest::PostmarkClient;
use postmark::api::email::SendEmailRequest;
use postmark::api::Body;
use postmark::Query;

#[tokio::test(flavor = "multi_thread")]
async fn send_email() {

    println!("Started test");

    let api_token = env::var("POSTMARK_API_TOKEN").expect("POSTMARK_API_TOKEN is not set");

    println!("Loaded env variable");

    let client = PostmarkClient::builder()
        .base_url("https://api.postmarkapp.com/")
        .token(api_token)
        .build();

    println!("Created client");

    let req = SendEmailRequest::builder()
        .from("dan@ourfructus.com")
        .to("customers@ourfructus.com")
        .body(Body::html("This is a basic e-mail test!".into()))
        .subject("Test")
        .build();

    println!("Creaed request");

    let resp = req.execute(&client).await;
    resp.unwrap();

}