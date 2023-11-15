// master_node.rs
//not complete, need to fill out some part(mappng logic, reducing logic, ...)

use std::env;
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};
use mapreduce::map_reduce_service_server::{MapReduceService, MapReduceServiceServer};
use mapreduce::{InputData, OutputData};

#[derive(Default)]
struct MapReduceServer;

#[tonic::async_trait]
impl MapReduceService for MapReduceServer {
    async fn map_function(&self, request: Request<InputData>) -> Result<Response<OutputData>, Status> {
        // Implement mapping logic here
        // ...

        Ok(Response::new(OutputData {
            // Set output data fields
        }))
    }

    async fn reduce_function(&self, request: Request<InputData>) -> Result<Response<OutputData>, Status> {
        // Implement reducing logic here
        // ...

        Ok(Response::new(OutputData {
            // Set output data fields
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::var("GRPC_MASTER_NODE_ADDR").unwrap_or_else(|_| "[::1]:50051".to_string());
    let addr = addr.parse()?;
    let map_reduce_server = MapReduceServer::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(MapReduceServiceServer::new(map_reduce_server))
        .serve(addr)
        .await?;

    Ok(())
}
