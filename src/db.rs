use cfg_if::cfg_if;

cfg_if! {

    if #[cfg(feature = "ssr")] {
        use sqlx::Connection;
		use sqlx::postgres::PgConnection;

        extern crate dotenv;

        use dotenv::dotenv;
        use std::env;

        pub async fn db() -> Result<PgConnection, sqlx::Error> {
            dotenv().ok();
            let pool = PgConnection::connect(&(std::env::var("DATABASE_URL").unwrap())).await?;
            Ok(pool)
        }
    }
}
