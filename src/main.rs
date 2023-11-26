// src/main.rs
#[macro_use]
extern crate diesel;

type StdErr = Box<dyn std::error::Error>;
mod logger;
mod models;
mod schema;

fn main() -> Result<(), StdErr> {
    // loads env variables from .env
    dotenv::dotenv()?;
    // initialize logger
    logger::init()?;
    // example
    assert_eq!("INFO", std::env::var("LOG_LEVEL").unwrap());

    Ok(())
}