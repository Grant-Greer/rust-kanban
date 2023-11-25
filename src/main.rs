// src/main.rs

type StdErr = Box<dyn std::error::Error>;
mod logger;
mod models;
fn main() -> Result<(), StdErr> {
    // loads env variables from .env
    dotenv::dotenv()?;
    // initialize logger
    logger::init()?;
    // example
    assert_eq!("INFO", std::env::var("LOG_LEVEL").unwrap());

    Ok(())
}