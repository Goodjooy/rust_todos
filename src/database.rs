use diesel::{mysql, r2d2::{ConnectionManager, Pool, PooledConnection}};
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

#[macro_export]
macro_rules! first_or_create {
    ($db:expr, $ty:ident, $t:ident, $tc:expr, $($filters : expr ),*) => {
        match $t
            $(.filter($filters))*
            .first::<$ty>($db){
            Ok(d) => d.into(),
            Err(_) => {
                to_rresult!(rs, 
                    diesel::insert_into(uesr_details)
                    .values(&$tc)
                    .execute($db)
                );
                $tc.into()
            }
        }
    };
}
#[macro_export]
macro_rules! update {
    (
        $db:expr,
        $ty:ident,
        $table:ident,
        $in_data:expr,
        filter => [ $($f:expr),* ],
        set => [$($s:expr),*]
    ) => {
        if let Ok(ud) = $table
            $(.filter($f))*
            .first::<$ty>($db)
        {
            diesel::update(&ud)
                $(.set($s))*
                .execute($db)
        } else {
            diesel::insert_into($table)
                .values(&$in_data)
                .execute($db)
        }
    };
    (
        $db:expr,
        $ty:ident,
        $table:ident,
        pk => $pk:expr,
        set => [$($s:expr),*]
    ) => {
        diesel::update($table.find($pk))
                $(.set($s))*
                .execute($db)
    };
    (
        $db:expr,
        $ty:ident,
        $table:ident,
        filter => [ $($f:expr),* ],
        set => [$($s:expr),*]
    )=>{
        diesel::update(
            $table
            $(.filter($f))*
            )
            $(.set($s))*
        .execute($db)
    }
}



#[macro_export]
macro_rules! load_first {
    ($db:expr,$ty:ident, $table:ident, filter=>[$( $f:expr ),*]) => {
        $table
        $(.filter($f))*
        .first::<$ty>($db)
    };
    ($db:expr,$ty:ident, $table:ident,pk=>$pk:expr ) => {
        $table
        .find($pk)
        .first::<$ty>($db)
    };
}

#[macro_export]
macro_rules! load {
    ($db:expr,$ty:ident, $table:ident, filter=>[$( $f:expr ),*]) => {
        $table
        $(.filter($f))*
        .load::<$ty>($db)
    };
}

#[macro_export]
macro_rules! insert_into {
    ($db:expr,$table:ident,$value:expr) => {
        diesel::insert_into($table).values($value).execute($db)
    };
}
