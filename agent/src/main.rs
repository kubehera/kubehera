mod grpc;
mod runtimes;

use crate::grpc::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    grpc::echo::run_grpc().await?;
    Ok(())
}