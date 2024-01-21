use axum::{
    routing::get,
    http::{ header::{ CONTENT_TYPE, ACCEPT }, HeaderValue },
    http::{ Method, header::AUTHORIZATION },
};
use rspc::integrations::httpz::Request;
use std::{ env, net::SocketAddr, sync::Arc };
use tower_http::cors::CorsLayer;

use s3::creds::Credentials;

mod routes;
#[allow(unused)]
mod prisma;
mod utils;

fn router(client: Arc<prisma::PrismaClient>, bucket: Arc<s3::Bucket>) -> axum::Router {

    let router: Arc<rspc::Router<routes::Ctx>> = routes::new().arced();

    axum::Router
        ::new()
        .route(
            "/",
            get(|| async {
                "Hello 'rspc'!"
            })
        )
        // .merge(routes::users::webhooks(client.clone()))
        .nest(
            "/rspc",
            router
            .endpoint(move |req: Request| {
                println!("Client requested operation '{}'", &req.uri().path());
                let token = req.headers().get(AUTHORIZATION).cloned();

                routes::Ctx { db: client.clone(), bucket: bucket.clone(), token }
            })
            .axum()
        )
        .layer(
            CorsLayer::new()
            .allow_credentials(true)
            .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT])
            .allow_methods([Method::GET, Method::POST])
            .allow_origin(
                env
                ::var("FRONTEND_URL")
                .unwrap_or("http://localhost:3000".to_string())
                .parse::<HeaderValue>()
                .unwrap()
            )
        )
}

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

    let client = match prisma::new_client().await {
		Ok(client) => client,
		Err(err) => panic!("Failed to connect to database: {}", err),
	};

    let account_id = std::env::var("CLOUDFLARE_R2_ACCOUNT_ID")
        .unwrap();

    let access_key = std::env::var("CLOUDFLARE_R2_ACCESS_KEY")
        .unwrap();

    let secret_key = std::env::var("CLOUDFLARE_R2_SECRET_KEY")
        .unwrap();

    let credentials = Credentials::new(
        Some(&access_key),
        Some(&secret_key),
        None,
        None,
        None
    );

    let bucket = s3::Bucket::new(
        "image-uploader-dev",
        s3::Region::R2 { account_id: account_id },
        credentials.unwrap()
    ).unwrap();

    let port = env::var("PORT").unwrap_or("4000".to_string());

    let addr = format!("[::]:{}", port).parse::<SocketAddr>().unwrap();

    println!("{} listening on http://{}", env!("CARGO_CRATE_NAME"), addr);

    axum::Server
        ::bind(&addr)
        .serve(
            router(
                Arc::new(client),
                Arc::new(bucket),
            ).into_make_service()
        )
        .with_graceful_shutdown(utils::axum_shutdown_signal()).await
        .expect("Error with HTTP server!");
}
