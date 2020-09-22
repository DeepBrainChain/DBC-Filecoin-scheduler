// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_SCHEDULER_TEST: ::grpcio::Method<super::empty::Empty, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/scheduler.Scheduler/Test",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SCHEDULER_TRY_ACCESS: ::grpcio::Method<super::scheduler::AccessResource, super::scheduler::ResourceResult> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/scheduler.Scheduler/TryAccess",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct SchedulerClient {
    client: ::grpcio::Client,
}

impl SchedulerClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SchedulerClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn test_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_SCHEDULER_TEST, req, opt)
    }

    pub fn test(&self, req: &super::empty::Empty) -> ::grpcio::Result<super::empty::Empty> {
        self.test_opt(req, ::grpcio::CallOption::default())
    }

    pub fn test_async_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_SCHEDULER_TEST, req, opt)
    }

    pub fn test_async(&self, req: &super::empty::Empty) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.test_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn try_access_opt(&self, req: &super::scheduler::AccessResource, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::scheduler::ResourceResult> {
        self.client.unary_call(&METHOD_SCHEDULER_TRY_ACCESS, req, opt)
    }

    pub fn try_access(&self, req: &super::scheduler::AccessResource) -> ::grpcio::Result<super::scheduler::ResourceResult> {
        self.try_access_opt(req, ::grpcio::CallOption::default())
    }

    pub fn try_access_async_opt(&self, req: &super::scheduler::AccessResource, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::scheduler::ResourceResult>> {
        self.client.unary_call_async(&METHOD_SCHEDULER_TRY_ACCESS, req, opt)
    }

    pub fn try_access_async(&self, req: &super::scheduler::AccessResource) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::scheduler::ResourceResult>> {
        self.try_access_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Scheduler {
    fn test(&mut self, ctx: ::grpcio::RpcContext, req: super::empty::Empty, sink: ::grpcio::UnarySink<super::empty::Empty>);
    fn try_access(&mut self, ctx: ::grpcio::RpcContext, req: super::scheduler::AccessResource, sink: ::grpcio::UnarySink<super::scheduler::ResourceResult>);
}

pub fn create_scheduler<S: Scheduler + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_TEST, move |ctx, req, resp| {
        instance.test(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_SCHEDULER_TRY_ACCESS, move |ctx, req, resp| {
        instance.try_access(ctx, req, resp)
    });
    builder.build()
}
