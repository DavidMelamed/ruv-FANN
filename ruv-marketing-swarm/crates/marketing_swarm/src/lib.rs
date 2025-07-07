use axum::{routing::get, Router};
use tonic::transport::Server as GrpcServer;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/marketing.rs"));
}


pub async fn serve() -> anyhow::Result<()> {
    let grpc = GrpcServer::builder()
        .add_service(proto::marketing_service_server::MarketingServiceServer::new(MyMarketingService));

    let app = Router::new().route("/", get(|| async { "swarm" }));

    tokio::spawn(async move {
        grpc.serve(([127,0,0,1], 50051).into()).await.unwrap();
    });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, app).await?;


    Ok(())
}

pub struct MyMarketingService;

#[tonic::async_trait]
impl proto::marketing_service_server::MarketingService for MyMarketingService {
    async fn create_campaign(&self, request: tonic::Request<proto::Campaign>) -> Result<tonic::Response<proto::Campaign>, tonic::Status> {
        Ok(tonic::Response::new(request.into_inner()))
    }
    async fn get_campaign(&self, request: tonic::Request<proto::Campaign>) -> Result<tonic::Response<proto::Campaign>, tonic::Status> {
        Ok(tonic::Response::new(request.into_inner()))
    }
    async fn update_campaign(&self, request: tonic::Request<proto::Campaign>) -> Result<tonic::Response<proto::Campaign>, tonic::Status> {
        Ok(tonic::Response::new(request.into_inner()))
    }
    async fn delete_campaign(&self, request: tonic::Request<proto::Campaign>) -> Result<tonic::Response<proto::Campaign>, tonic::Status> {
        Ok(tonic::Response::new(request.into_inner()))
    }
}
