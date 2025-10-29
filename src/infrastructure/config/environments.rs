use once_cell::sync::Lazy;
use std::env;

pub struct Environments {
    pub logs_db_url: String,
    pub server_port: u16,
}

pub static ENVIRONMENTS: Lazy<Environments> = Lazy::new (|| {
    dotenvy::dotenv().ok();

    println!("🚀 Starting up the server...");

    Environments {
        server_port: get_environment("SERVER_PORT")
            .parse::<u16>()
            .expect("FATAL_ERROR: SERVER_PORT must be a valid port number between (0-65535)!"),
        logs_db_url: get_environment("LOGS_DB_URL"),
    }
});

fn get_environment(key: &'static str) -> String {
    env::var(key)
        .unwrap_or_else(|_| panic!("FATAL_ERROR: Env var {key} not found!"))
}
