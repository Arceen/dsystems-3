use sea_orm::{ConnectOptions, Database};
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_uri = env::var("DB_URI").expect("Could not find the matching env");
    println!("db_url: {db_uri}");
    let mut opt = ConnectOptions::new(db_uri);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public");
    let db = Database::connect(opt).await.unwrap();
}