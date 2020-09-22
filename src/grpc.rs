use crate::common::*;
use crate::cond::*;
use crate::protos::scheduler::*;
use crate::protos::scheduler_grpc;
use futures::TryFutureExt;
use grpcio::{RpcContext, UnarySink};
use protobuf::well_known_types::Empty;

impl scheduler_grpc::Scheduler for Scheduler {
    fn test(&mut self, ctx: RpcContext, _req: Empty, sink: UnarySink<Empty>) {
        ctx.spawn(sink.success(Empty::new()).unwrap_or_else(|_| ()))
    }

    fn try_access(
        &mut self,
        ctx: RpcContext,
        req: AccessResource,
        sink: UnarySink<ResourceResult>,
    ) {
        println!("{:#?}", req);

        let mut data = ResourceResult::new();
        data.set_token(1);

        let f = sink.success(data).unwrap_or_else(|_| ());
        ctx.spawn(f)
    }
}
