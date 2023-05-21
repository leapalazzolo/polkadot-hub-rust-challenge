mod models;
mod presentation;
mod repository;
mod schema;
mod service;
mod utils;
use diesel::prelude::*;
use diesel::SqliteConnection;

use std::env;

fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    let repository = repository::HouseRepository { conn };
    let service = service::HouseService::new(repository);

    let mut gui = presentation::GUI::new(service);
    gui.build();
    gui.show();
}
