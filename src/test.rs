mod common;
mod cond;
mod config;
mod grpc;
mod protos;

use grpcio::{ChannelBuilder, EnvBuilder};
use log::*;
use protos::scheduler::{AccessResource, Empty, ResourceToken};
use protos::scheduler_grpc::SchedulerClient;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn fake_p1() {
    thread::sleep(Duration::from_secs(30));
}

fn main() {
    env_logger::init();

    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env).connect("localhost:3000");
    let client = SchedulerClient::new(channel);

    let mut req = AccessResource::new();
    req.set_name("P1".to_string());
    let token1 = client.try_access(&req).unwrap();
    info!("{:?}", token1);

    thread::sleep(Duration::from_secs(5));

    let empty = Empty::new();
    let r = client.test(&empty);
    info!("{:?}", r);

    let mut ping = ResourceToken::new();
    ping.set_token(token1.get_token());
    let r = client.ping(&ping);
    info!("{:?}", r);

    let empty = Empty::new();
    let r = client.test(&empty);
    info!("{:?}", r);
}
