//! # proto
//! Rust definitions for microstack gRPC protocol.

pub use tonic;

/// Protocol version string, compiled from git revision and semver.
pub static VERSION: &str = env!("VERGEN_GIT_SEMVER");

pub mod health {
    //! Health service definitions.
    tonic::include_proto!("grpc.health.v1");

	use std::{future::Future, pin::Pin};

    use tokio_stream::wrappers::ReceiverStream;
    use tonic::{Request, Response, Status};

    use self::{
        health_check_response::ServingStatus,
        health_server::{Health, HealthServer},
    };

    /// Provides a basic versioning service for verifying services can interact without protocol collision.
    pub struct HealthProvider {
        handler: Box<fn () -> (dyn Future<Output = ServingStatus> + Send + 'static)>,
    }

    impl HealthProvider {
        /// Shorthand for `HealthServer::new(HealthProvider)`.
        pub fn as_service(f: fn () -> (dyn Future<Output = ServingStatus> + Send + 'static)) -> HealthServer<HealthProvider> {
            HealthServer::new(Self {
                handler: Box::new(f),
            })
        }
    }

    #[tonic::async_trait]
    impl Health for HealthProvider {
        /// Type alias for response type of `Watch`.
        type WatchStream = ReceiverStream<Result<HealthCheckResponse, Status>>;

        async fn check(
            &self,
            request: Request<HealthCheckRequest>,
        ) -> Result<Response<HealthCheckResponse>, Status> {
            let reply = Response::new(HealthCheckResponse {
                status: (self.handler)().await as i32,
            });
            Ok(reply)
        }

        async fn watch(
            &self,
            request: Request<HealthCheckRequest>,
        ) -> Result<Response<Self::WatchStream>, Status> {
            todo!()
        }
    }
}

pub mod version {
    //! Version service definitions.
    tonic::include_proto!("microstack.version");

    use self::version_service_server::{VersionService, VersionServiceServer};
    use super::VERSION;
    use tonic::{Request, Response, Status};

    /// Provides a basic versioning service for verifying services can interact without protocol collision.
    #[derive(Default, Debug)]
    pub struct VersionProvider;

    impl VersionProvider {
        /// Shorthand for `VersionServiceServer::new(VersionProvider)`.
        pub fn as_service() -> VersionServiceServer<VersionProvider> {
            VersionServiceServer::new(Self)
        }
    }

    #[tonic::async_trait]
    impl VersionService for VersionProvider {
        async fn get_version(
            &self,
            _: Request<()>,
        ) -> Result<Response<GetVersionResponse>, Status> {
            Ok(Response::new(GetVersionResponse {
                version: VERSION.to_string(),
            }))
        }
    }
}

pub mod products {
    //! Products service definitions.
    tonic::include_proto!("microstack.products");
}
