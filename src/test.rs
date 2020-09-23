mod common;
mod cond;
mod config;
mod grpc;
mod protos;

use grpcio::{ChannelBuilder, EnvBuilder};
use log::*;
use protos::scheduler::{AccessResource, Empty, ResourceToken};
use protos::scheduler_grpc::SchedulerClient;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct LiveGuard {
    client: Arc<SchedulerClient>,
    ping_token: Arc<ResourceToken>,
    thread_handle: Option<thread::JoinHandle<()>>,
    thread_exit_sender: Sender<()>,
}

impl LiveGuard {
    fn new(client: Arc<SchedulerClient>, token: Arc<ResourceToken>) -> Self {
        let (tx, rx) = channel();
        let mut guard = Self {
            client: client.clone(),
            ping_token: token.clone(),
            thread_handle: None,
            thread_exit_sender: tx,
        };

        let handle = thread::spawn(move || loop {
            if rx.try_recv().is_ok() {
                trace!("LiveGuard {} destroyed", token.get_token());
                break;
            }

            client.ping(&*token).expect("Rpc ping error");
            thread::sleep(Duration::from_secs(5));
        });

        guard.thread_handle = Some(handle);
        guard
    }
}

impl std::ops::Drop for LiveGuard {
    fn drop(&mut self) {
        self.thread_exit_sender.send(()).unwrap();
        self.thread_handle
            .take()
            .unwrap()
            .join()
            .expect("Thread join failed");
        self.client
            .remove_guard(&*self.ping_token)
            .expect("Rpc remove_guard error");
    }
}

fn fake_p1() {
    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env).connect("localhost:3000");
    let client = SchedulerClient::new(channel);

    let mut req = AccessResource::new();
    req.set_name("P1".to_string());
    let mut token = client.try_access(&req).unwrap();

    let _guard = LiveGuard::new(Arc::new(client), Arc::new(token.take_token()));

    thread::sleep(Duration::from_secs(30));
}

fn main() {
    env_logger::init();

    fake_p1();

    // let env = Arc::new(EnvBuilder::new().build());
    // let channel = ChannelBuilder::new(env).connect("localhost:3000");
    // let client = SchedulerClient::new(channel);
    //
    // let mut req = AccessResource::new();
    // req.set_name("P1".to_string());
    // let token1 = client.try_access(&req).unwrap();
    // info!("{:?}", token1);
    //
    // thread::sleep(Duration::from_secs(5));
    //
    // let empty = Empty::new();
    // let r = client.test(&empty);
    // info!("{:?}", r);

    // let mut ping = ResourceToken::new();
    // ping.set_token(token1.get_token());
    // let r = client.ping(&ping);
    // info!("{:?}", r);
    //
    // let empty = Empty::new();
    // let r = client.test(&empty);
    // info!("{:?}", r);
}
