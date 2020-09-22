pub mod scheduler;
pub mod scheduler_grpc;

pub mod empty {
    pub use protobuf::well_known_types::Empty;
}
