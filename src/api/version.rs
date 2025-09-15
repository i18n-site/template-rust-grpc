use volo_gen::api::Version;
use volo_grpc::Response;

pub async fn version(
  _: volo_grpc::Request<volo_gen::api::Void>,
) -> Result<Response<Version>, volo_grpc::Status> {
  Ok(Response::new(Version {
    major: const_str::parse!(env!("CARGO_PKG_VERSION_MAJOR"), u32),
    minor: const_str::parse!(env!("CARGO_PKG_VERSION_MINOR"), u32),
    patch: const_str::parse!(env!("CARGO_PKG_VERSION_PATCH"), u32),
  }))
}
