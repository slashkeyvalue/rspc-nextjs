use axum::{
    routing::get,
    http::{ header::{ CONTENT_TYPE, ACCEPT }, HeaderValue },
    http::{ Method, header::AUTHORIZATION },
};
use rspc::integrations::httpz::Request;
use std::{ env, net::SocketAddr, sync::Arc };
use tower_http::cors::CorsLayer;

mod routes;
#[allow(unused)]
mod prisma;
mod utils;

fn router(client: Arc<prisma::PrismaClient>) -> axum::Router {

    let router = routes::new().arced();

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

                routes::Ctx { db: client.clone(), token }
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

    println!( "0" );

    dotenv::dotenv().ok();

    println!( "1" );

    let client = match prisma::new_client().await {
		Ok(client) => client,
		Err(err) => panic!("Failed to connect to database: {}", err),
	};

    println!( "2" );

    let port = env::var("PORT").unwrap_or("9000".to_string());

    println!( "3" );

    let addr = format!("[::]:{}", port).parse::<SocketAddr>().unwrap();

    println!( "4" );

    println!("{} listening on http://{}", env!("CARGO_CRATE_NAME"), addr);

    axum::Server
        ::bind(&addr)
        .serve(router(Arc::new(client)).into_make_service())
        .with_graceful_shutdown(utils::axum_shutdown_signal()).await
        .expect("Error with HTTP server!");
}
