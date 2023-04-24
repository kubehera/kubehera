
use crate::runtimes::ebpf::ebpf::*;
use std::error::Error;
use tokio_stream::{StreamExt, StreamMap, Stream};
use tonic::transport::Channel;
use tokio::sync::mpsc;
use std::thread;

use std::pin::Pin;

use crate::runtimes::ebpf::ebpf::pb::{project_service_client::ProjectServiceClient, AgentStatus, ProjectMessage,ProjectData};


pub struct Project {
    client: ProjectServiceClient<Channel>,
}
impl Project {
    pub async fn new()-> Project {
        let client = ProjectServiceClient::connect("http://10.30.72.188:1234").await.unwrap();
        Project{
            client: client,
        }
    }

    async fn send_project_message_throttle(&mut self,mut rx: mpsc::Receiver<ProjectMessage>) -> Result<(), Box<dyn Error>>{
    
        let request = Box::pin(async_stream::stream! {
            while let Some(item) = rx.recv().await {
                yield item;
            }
        }) as Pin<Box<dyn Stream<Item = ProjectMessage> + Send>>;
    
        match self.client.send_project_message(request).await{
            Ok(response) => println!("get status of project {:?}",response.into_inner()),
            Err(err) => println!("something went wrong: {:?}", err),
        }
        Ok(())
    
    }
    
    pub async fn run_grpc(&mut self) -> Result<(),Box<dyn std::error::Error>> {
        let (tx, rx) = mpsc::channel::<ProjectMessage>(10);
        // Echo stream that sends up to `usize::MAX` requests. One request each 2s.
        // Exiting client with CTRL+C demonstrate how to distinguish broken pipe from
        // graceful client disconnection (above example) on the server side.
        tokio::spawn(async move {
           let _ = run_ebpf(tx);
        });

        println!("\r\nBidirectional stream echo (kill client with CTLR+C):");
        self.send_project_message_throttle(rx).await?;
        Ok(())
    }
}
