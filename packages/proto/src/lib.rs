//! # proto
//! Rust definitions for microstack gRPC protocol.

pub use tonic;

pub mod health {
	//! Health service definitions.
	tonic::include_proto!("grpc.health.v1");
}

pub mod version {
	//! Version service definitions.
	tonic::include_proto!("microstack.version");
}
