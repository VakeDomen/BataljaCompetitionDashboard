use r2d2::{self, Pool, PooledConnection, Error as R2D2Error};
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use std::env;
use std::sync::Mutex;
use once_cell::sync::Lazy;

type DbConn = PooledConnection<ConnectionManager<MysqlConnection>>;

static POOL: Lazy<Mutex<Pool<ConnectionManager<MysqlConnection>>>> = Lazy::new(|| {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    Mutex::new(pool)
});

pub fn establish_connection() -> Result<DbConn, R2D2Error> {
    let pool = POOL.lock().unwrap();
    pool.get()
}