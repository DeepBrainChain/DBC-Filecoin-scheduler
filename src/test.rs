mod common;
mod cond;
mod grpc;
mod protos;

use grpcio::{ChannelBuilder, EnvBuilder};
use log::*;
use protos::scheduler::Empty;
use protos::scheduler_grpc::SchedulerClient;
use std::sync::Arc;

fn main() {
    env_logger::init();

    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env).connect("localhost:3000");
    let client = SchedulerClient::new(channel);

    let empty = Empty::new();
    let r = client.test(&empty);
    info!("{:?}", r);
}
