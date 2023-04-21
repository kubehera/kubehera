pub mod pb {
    tonic::include_proto!("project");
}

use std::error::Error;
//use async_channel::{unbounded, TryRecvError};
use std::time::Duration;
use tokio_stream::{StreamExt, StreamMap, Stream};
use tonic::transport::Channel;
use tokio::sync::mpsc;
use std::pin::Pin;

use pb::{project_service_client::ProjectServiceClient, AgentStatus, Project, ProjectMessage,ProjectData};

/*fn project_messages_iter(msgs: Vec<String>) -> impl Stream<Item = ProjectMessage> {
    tokio_stream::iter(msgs).map(|i| ProjectMessage {
        message: format!("msg {}", i),
    })
}*/

async fn send_project_message_throttle(client: &mut ProjectServiceClient<Channel>, dur: Duration) -> Result<(), Box<dyn Error>>{
    //let (s, r) = unbounded();
    //s.send("ttttttt");
    //s.recv();
    let (tx, mut rx) = mpsc::channel::<ProjectMessage>(10);


        ///tokio::spawn(async move {
        ///         tx1.send(1).await.unwrap();
        ///
        ///         // This value will never be received. The send may or may not return
        ///         // `Err` depending on if the remote end closed first or not.
        ///         let _ = tx1.send(2).await;
        ///     });
    tokio::spawn(async move {
        tx.send(ProjectMessage{
            name: "test".to_string(),
            status: 0,
            message: format!("msg"),
        }).await.unwrap();
    });


    let rx = Box::pin(async_stream::stream! {
        while let Some(item) = rx.recv().await {
            yield item;
        }
    }) as Pin<Box<dyn Stream<Item = ProjectMessage> + Send>>;

/*
    let in_stream = tokio_stream::FromStream(r).map(|i| ProjectMessage {
        name: "test".to_string(),
        status: 0,
        message: format!("msg {}", i),
    });
 */
   // let in_stream = project_messages_iter().throttle(dur);

    match client.send_project_message(rx).await{
        Ok(response) => println!("get status of project {:?}",response.into_inner()),
        Err(err) => println!("something went wrong: {:?}", err),
    }
    Ok(())

/*   let mut resp_stream = response.into_inner();

    while let Some(received) = resp_stream.next().await {
        let received = received.unwrap();
        println!("\treceived message: `{}`", received.message);
    } */ 
}

pub async fn run_grpc() -> Result<(),Box<dyn std::error::Error>> {
    let mut client = ProjectServiceClient::connect("http://10.30.72.189:1234").await?;
    // Echo stream that sends up to `usize::MAX` requests. One request each 2s.
    // Exiting client with CTRL+C demonstrate how to distinguish broken pipe from
    // graceful client disconnection (above example) on the server side.
    println!("\r\nBidirectional stream echo (kill client with CTLR+C):");
    send_project_message_throttle(&mut client, Duration::from_secs(2)).await?;
    Ok(())
}