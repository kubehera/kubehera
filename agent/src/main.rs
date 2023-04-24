mod grpc;
mod runtimes;

use crate::grpc::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //grpc::echo::run_grpc().await?;
    let mut project = grpc::project::Project::new().await;
    project.run_grpc().await?;
    Ok(())
}