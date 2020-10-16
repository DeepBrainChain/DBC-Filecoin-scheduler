mod common;
mod cond;
mod config;
mod grpc;
mod protos;

use clap::App;
// use config::Config;
use futures::channel::oneshot;
use futures::executor::block_on;
use grpcio::{Environment, ServerBuilder};
use log::*;
use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

fn main() {
    let _m = App::new("filecoin-scheduler")
        // .arg(
        //     Arg::with_name("config")
        //         .short("-c")
        //         .help("config file")
        //         .takes_value(true),
        // )
        .version("2.0.0")
        .author("sbw <sbw@sbw.so>")
        .get_matches();

    if std::env::var("RUST_LOG").is_err() {
        if cfg!(debug_assertions) {
            std::env::set_var("RUST_LOG", "trace");
        } else {
            std::env::set_var("RUST_LOG", "debug");
        }
    }

    env_logger::init();

    // let config = Config::from_config(
    //     m.value_of("config")
    //         .unwrap_or("/etc/filecoin-scheduler.yaml"),
    // );
    // debug!("\n{}", serde_yaml::to_string(&config).unwrap());

    let service = protos::scheduler_grpc::create_scheduler(common::Scheduler);
    let env = Arc::new(Environment::new(5));
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        // .bind(&config.bind_host, config.bind_port)
        .bind("localhost", 3000)
        .build()
        .unwrap();
    server.start();
    for (ref host, port) in server.bind_addrs() {
        info!("listen: {}:{}", host, port);
    }

    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = block_on(rx);
    let _ = block_on(server.shutdown());
}
