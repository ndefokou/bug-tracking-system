use axum::Server;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = create_routes(); // Include your route
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
