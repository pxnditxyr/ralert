use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::env;

pub struct Environments {
    pub server_port: u16,
    pub database_url: String,
    pub jwt_secret: String,
}

pub static ENVIRONMENTS: Lazy<Environments> = Lazy::new(|| {
    dotenv().ok();

    println!("🚀 Starting up the server...");

    Environments {
        server_port: get_environment("SERVER_PORT")
            .parse::<u16>()
            .expect("FATAL_ERROR: SERVER_PORT must be a valid port number between (0-65535)!"),
        database_url: get_environment("DATABASE_URL"),
        jwt_secret: get_environment("JWT_SECRET"),
    }
});

fn get_environment(key: &'static str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("FATAL_ERROR: Env var {key} not found!"))
}
