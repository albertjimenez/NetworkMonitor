pub mod environment_variables {
    pub mod common {
        use std::env;

        pub const NATS_URL: &str = "NATS_URL";

        pub fn get_nats_url() -> String {
            env::var(NATS_URL).expect("Environment variable NATS_URL is not present")
        }
    }

    pub mod database {
        use std::env;

        pub const DB_URL: &str = "DATABASE_URL";

        pub fn get_db_url() -> String {
            env::var(DB_URL).expect("Environment variable DATABASE_URL is not present")
        }
    }
}
