#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
}
// using Clone to retain the ownership of Config struct as we are returning Clone Debug in impl

// Updated it as per latest changes
impl Config {
    pub fn new() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

        Config {
            database_url,
        }
    }
}
