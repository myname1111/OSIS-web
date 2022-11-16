use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;
use dotenvy::dotenv;
use std::env;

pub mod division;
pub mod member;
pub mod image;
mod schema;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn database_url() -> String {
    dotenv().ok();

    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_connection_pool() -> DbPool {
    let url = database_url();
    let manager = ConnectionManager::<PgConnection>::new(url);

    Pool::builder()
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn get_conn_from_pool(pool: actix_web::web::Data<DbPool>) -> DbConnection {
    pool.get().expect("couldn't get db connection from pool")
}
