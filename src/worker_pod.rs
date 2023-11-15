// worker_pod.rs
// need to fill how to recup data and manipulate it afterward like if it's mapping replace it on job queue at the end and if 
//it's reducing step figure out how to send it back to the client (may be collecting all the reduced file somehow before sending it back) 
// ...

use tonic::transport::Channel;
use mapreduce::map_reduce_service_client::MapReduceServiceClient;
use mapreduce::{InputData, OutputData};

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let master_node_addr = env::var("GRPC_MASTER_NODE_ADDR").unwrap_or_else(|_| "[::1]:50051".to_string());
    let channel = Channel::from_shared(master_node_addr)?.connect().await?;
    let mut client = MapReduceServiceClient::new(channel);

    // Example of calling MapFunction
    let input_data = InputData {
        // Set input data fields
    };
    let response = client.map_function(Request::new(input_data)).await?;

    // Process the response
    let output_data = response.into_inner();
    // ...

    Ok(())
}
