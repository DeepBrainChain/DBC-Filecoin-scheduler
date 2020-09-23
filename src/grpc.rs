use crate::common::*;
use crate::cond;
use crate::protos::scheduler::*;
use crate::protos::scheduler_grpc;
use futures::TryFutureExt;
use grpcio::{RpcContext, UnarySink};
use log::*;

impl scheduler_grpc::Scheduler for Scheduler {
    fn test(&mut self, ctx: RpcContext, _req: Empty, sink: UnarySink<Empty>) {
        cond::show_debug_info();

        ctx.spawn(sink.success(Empty::new()).unwrap_or_else(|_| ()))
    }

    fn try_access(
        &mut self,
        ctx: RpcContext,
        req: AccessResource,
        sink: UnarySink<ResourceResult>,
    ) {
        trace!("grpc: Trying to access {}", req.get_name());

        let mut data = ResourceResult::new();
        if let Some(tok) = cond::try_access(req.get_name()) {
            let mut rtok = ResourceToken::new();
            rtok.set_token(tok);
            data.set_token(rtok);
        }

        let f = sink.success(data).unwrap_or_else(|_| ());
        ctx.spawn(f)
    }

    fn remove_guard(&mut self, ctx: RpcContext, req: ResourceToken, sink: UnarySink<ExecResult>) {
        trace!("grpc: Remove guard {:?}", req);

        let mut data = ExecResult::new();
        if let Some(r) = cond::remove_guard(req.get_token()) {
            data.set_result(r);
        }

        let f = sink.success(data).unwrap_or_else(|_| ());
        ctx.spawn(f)
    }

    fn ping(&mut self, ctx: RpcContext, req: ResourceToken, sink: UnarySink<ExecResult>) {
        trace!("grpc: Ping token {}", req.get_token());

        let mut data = ExecResult::new();
        if cond::ping(req.get_token()) {
            data.set_result(true);
        }

        let f = sink.success(data).unwrap_or_else(|_| ());
        ctx.spawn(f)
    }
}
