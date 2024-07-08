use anyhow::Result;
use tonic::{transport::Endpoint, Request, Status};
use tonic_demo::pb::{greeter_client::GreeterClient, HelloRequest};

#[tokio::main]
async fn main() -> Result<()> {
    let channel = Endpoint::from_static("http://localhost:8080")
        .connect()
        .await?;
    let mut client = GreeterClient::with_interceptor(channel, intercept);
    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });
    println!("extensions-{:?}", request.extensions().get::<MyExtension>());
    let response = client.say_hello(request).await?;
    println!("RESPONSE={:?}", response);
    Ok(())
}

fn intercept(mut req: Request<()>) -> Result<Request<()>, Status> {
    req.extensions_mut().insert(MyExtension {
        some_piece_data: "MyExtension".to_string(),
    });
    println!("Intercepting request: {:?}", req);
    Ok(req)
}

#[derive(Debug)]
struct MyExtension {
    some_piece_data: String,
}
