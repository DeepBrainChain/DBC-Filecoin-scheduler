mod common;
mod cond;

mod grpc;
mod protos;

use futures::channel::oneshot;
use futures::executor::block_on;
use grpcio::{Environment, ServerBuilder};
use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

fn main() {
    let service = protos::scheduler_grpc::create_scheduler(common::Scheduler);
    let env = Arc::new(Environment::new(1));
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("0.0.0.0", 5100)
        .build()
        .unwrap();
    server.start();

    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = block_on(rx);
    let _ = block_on(server.shutdown());
}
