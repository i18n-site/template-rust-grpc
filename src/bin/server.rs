use std::net::SocketAddr;

use aok::{OK, Void};
use grpc::S;
use volo_grpc::{
  codec::compression::{CompressionEncoding::Gzip, GzipConfig},
  server::{Server, ServiceBuilder},
};

genv::s!(GRPC_ADDR:String | "0.0.0.0:3333".into());

#[volo::main]
async fn main() -> Void {
  loginit::init();

  let addr: SocketAddr = GRPC_ADDR.parse().unwrap();
  let addr = volo::net::Address::from(addr);

  tracing::info!("LISTEN ON {}", addr);

  let srv = ServiceBuilder::new(volo_gen::api::ApiServer::new(S))
    .send_compressions(vec![Gzip(Some(GzipConfig::default()))])
    .accept_compressions(vec![Gzip(None)])
    .build();

  xerr::log!(
    Server::new()
      .add_service(srv)
      .layer_front(volo_layer::Log)
      .run(addr)
      .await
  );
  OK
}
