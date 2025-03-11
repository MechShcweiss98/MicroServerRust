use server::server::server_initial;

mod handler;
mod middleware;
mod model;
mod router;
mod server;
mod service;
mod repository;
mod utils;

#[tokio::main]
async fn main() {
  println!("listening on 4500");
  dotenvy::dotenv().expect("Unable to access .env file");
  server_initial().await;
}
