use std::error::Error;

use proto::{
    health::{health_check_response::ServingStatus, HealthProvider},
    products::{
        products_service_server::{ProductsService, ProductsServiceServer},
        ListProductsResponse, Product,
    },
    tonic::{transport::Server, Request, Response, Status},
    version::VersionProvider,
};

#[derive(Debug, Default)]
pub struct ProductsProvider {}

#[proto::tonic::async_trait]
impl ProductsService for ProductsProvider {
    async fn list_products(
        &self,
        _: Request<()>,
    ) -> Result<Response<ListProductsResponse>, Status> {
        let reply = ListProductsResponse {
            products: vec![Product {
                id: "1".into(),
                name: "Banana".into(),
                description: "shiny banana ooo".into(),
            }],
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = ProductsProvider::default();

    Server::builder()
        .add_service(HealthProvider::as_service(|| {
            Box::pin(async { ServingStatus::Serving })
        }))
        .add_service(VersionProvider::as_service())
        .add_service(ProductsServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
