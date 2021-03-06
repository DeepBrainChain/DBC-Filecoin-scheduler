mod protos;

use log::*;
use protos::scheduler::ResourceToken;
use protos::scheduler_grpc::SchedulerClient;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[macro_export]
macro_rules! wait_cond {
    ($cond:expr, $poll_time:expr, $keep_live_time:expr) => {{
        let env = Arc::new(EnvBuilder::new().build());
        let channel = ChannelBuilder::new(env).connect("127.0.0.1:3000");
        let client = SchedulerClient::new(channel);

        let req_name = format!(
            "{}-{}",
            &$cond,
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let mut req = AccessResource::new();
        req.set_request_resource($cond);
        req.set_name(req_name.clone());

        let (client, token) = loop {
            let mut r = client.try_access(&req).expect("Rpc try_access error");
            if r.has_token() {
                debug!("{} got token {}", &req_name, r.get_token().get_token());
                break (Arc::new(client), Arc::new(r.take_token()));
            }

            thread::sleep(Duration::from_secs($poll_time));
        };

        LiveGuard::new(client, token, $keep_live_time)
    }};
}

pub struct LiveGuard {
    client: Arc<SchedulerClient>,
    ping_token: Arc<ResourceToken>,
    thread_handle: Option<thread::JoinHandle<()>>,
    thread_exit_sender: Sender<()>,
}

impl LiveGuard {
    #[allow(dead_code)]
    fn new(
        client: Arc<SchedulerClient>,
        token: Arc<ResourceToken>,
        keep_live_timeout: u64,
    ) -> Self {
        let (tx, rx) = channel();
        let mut guard = Self {
            client: client.clone(),
            ping_token: token.clone(),
            thread_handle: None,
            thread_exit_sender: tx,
        };

        let handle = thread::spawn(move || loop {
            match rx.try_recv() {
                Ok(_) => {
                    debug!("LiveGuard {} destroyed", token.get_token());
                    break;
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    warn!("recv error");
                    break;
                }
            }

            thread::sleep(Duration::from_secs(keep_live_timeout));
            trace!("rpc ping token {}", token.get_token());
            client.ping(&*token).expect("Rpc ping error");
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
