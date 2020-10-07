mod common;
mod cond;
mod config;
mod grpc;
mod protos;

use crate::wait_cond;
use grpcio::{ChannelBuilder, EnvBuilder};
use log::*;
use protos::scheduler::{AccessResource, ResourceToken};
use protos::scheduler_grpc::SchedulerClient;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};

fn fake_p1() {
    info!("fake_p1 started");
    let _guard = wait_cond!("P1".to_string(), 3, 5);

    thread::sleep(Duration::from_secs(40));
}

fn bad_p1() {
    info!("bad_p1 started");
    let _guard = wait_cond!("P1".to_string(), 3, 5);

    thread::sleep(Duration::from_secs(10));
}

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "trace");
    }

    env_logger::init();

    let mut hs: Vec<_> = (0..2).map(|_| thread::spawn(|| fake_p1())).collect();
    hs.push(thread::spawn(|| bad_p1()));
    hs.append(&mut (0..2).map(|_| thread::spawn(|| fake_p1())).collect());

    for h in hs {
        h.join().unwrap();
    }
}
