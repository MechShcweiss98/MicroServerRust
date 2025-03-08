// use sea_orm::{Database, DatabaseConnection};
use server::server::server_initial;

mod handler;
mod model;
mod router;
mod server;
mod utils;
mod middleware;

#[tokio::main]
async fn main() {
  //expose the enviramoent variables
  dotenvy::dotenv().expect("Unable to access .env file");
  // let db: DatabaseConnection = Database::connect("postgres://user:password@localhost:5432/data_base").await.unwrap();

  server_initial().await;
  println!("listening on 4500");
}
