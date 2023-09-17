#![allow(unused_imports)]
use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder as ReflectionBuilder;

use kvproto;

#[derive(Debug, Default)]
pub struct TiKVServer {}

#[tonic::async_trait]
impl kvproto::tikvpb::tikv_server::Tikv for TiKVServer {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let tikv_server = TiKVServer::default();

    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(kvproto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(kvproto::tikvpb::tikv_server::TikvServer::new(tikv_server))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
