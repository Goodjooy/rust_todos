use diesel::{
    mysql,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use rocket::fairing::Result;
use std::env;

pub struct DatabaseConnection {
    db: Pool<ConnectionManager<mysql::MysqlConnection>>,
}

impl DatabaseConnection {
    pub fn new() -> Result<Self, String> {
        dotenv::dotenv().ok();
        let db_url =
            env::var("DATABASE_URL").or_else(|e| Err(format!("DATABASE_URL Not Found | {}", e)))?;
        let max_conn = env::var("MAX_CONN")
            .unwrap_or("16".into())
            .trim()
            .parse()
            .unwrap_or(16);

        let manager = ConnectionManager::new(&db_url);

        let pool = Pool::builder()
            .max_size(max_conn)
            .build(manager)
            .or_else(|e| Err(format!("Construct DB Conncetion Pool Error | {}", e)))?;

        Ok(Self { db: pool })
    }

    pub fn get(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<mysql::MysqlConnection>>, String> {
        let res = self
            .db
            .get()
            .or_else(|e| Err(format!("Error From Get Connection: {}", e)))?;
        Ok(res)
    }
    pub fn lock(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<mysql::MysqlConnection>>, String> {
        self.get()
    }
}
