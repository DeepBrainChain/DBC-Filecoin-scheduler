use futures::future::{self, Ready};
use futures::prelude::*;
use std::{io, net::SocketAddr};
use tarpc::context;
use tarpc::server::{self, Channel};
use tokio_serde::formats::Json;

mod cond;
use cond::*;

#[tarpc::service]
pub trait Scheduler {
    async fn get_cond(cond: String) -> Option<u64>;
    async fn remove_guard(token: u64) -> Option<bool>;
}

#[derive(Clone)]
struct SchedulerServer;

impl Scheduler for SchedulerServer {
    type GetCondFut = Ready<Option<u64>>;
    type RemoveGuardFut = Ready<Option<bool>>;

    fn get_cond(self, _: context::Context, cond: String) -> Self::GetCondFut {
        future::ready(try_access(cond))
    }

    fn remove_guard(self, _: context::Context, token: u64) -> Self::RemoveGuardFut {
        future::ready(remove_guard(token))
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        if cfg!(debug_assertions) {
            std::env::set_var("RUST_LOG", "scheduler=trace");
        }
    }

    env_logger::init();

    let server_addr: SocketAddr = "127.0.0.1:6000".parse().unwrap();

    tarpc::serde_transport::tcp::listen(&server_addr, Json::default)
        .await?
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .map(|channel| {
            let server = SchedulerServer;
            channel.respond_with(server.serve()).execute()
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
