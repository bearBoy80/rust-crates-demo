use anyhow::Result;
use tonic::{transport::Server, Request, Response, Status};
use tonic_demo::pb::{
    greeter_server::{Greeter, GreeterServer},
    helloworld, HelloReply, HelloRequest,
};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let greeter = MyGreeter::default();
    let svc = GreeterServer::with_interceptor(greeter, intercept);
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}
#[derive(Debug, Default)]
struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let reply = helloworld::HelloReply {
            message: format!("hello world :{}", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}
