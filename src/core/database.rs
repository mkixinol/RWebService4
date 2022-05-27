
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Clone)]
pub struct RWDatabase {
    url: String,
    channel: usize,
    pool: Option<PgPool>
}


impl RWDatabase {
    pub fn new(
        user: &str,
        password: &str,
        host: &str,
        port: &str,
        database: &str,
        channel: usize
    ) -> Self {
        Self {
            url: format!(
                "postgres://{}:{}@{}:{}/{}",
                user,
                password,
                host,
                port,
                database,
            ),
            channel: channel,
            pool: None
        }
    }

    pub async fn connect(&mut self) -> bool {
        self.pool = PgPoolOptions::new()
            .max_connections(self.channel.try_into().unwrap())
            .connect(&self.url)
            .await
            .ok();

        self.pool.is_some()
    }

    pub fn get_pool(&self) -> Option<PgPool> {
        self.pool.clone()
    }
}