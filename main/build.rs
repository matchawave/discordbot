use std::env;

const ENVIRONMENTVARIABLES: [&str; 4] = [
    "BACKEND_URL",
    "LAVALINK_HOST",
    "LAVALINK_PORT",
    "LAVALINK_PASSWORD",
];

fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    for &var in &ENVIRONMENTVARIABLES {
        let value = env::var(var).unwrap_or_default();
        println!("cargo:rustc-env={}={}", var, value);
    }
}
