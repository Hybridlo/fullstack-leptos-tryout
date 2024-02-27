pub mod categories;
pub mod item;

use std::{env, convert::Infallible, future::{Ready, ready}};
use actix_web::FromRequest;
use derive_more::{Error, From, Display};

use sqlx::postgres::{PgPoolOptions, PgPool};

#[derive(Clone)]
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub async fn new() -> Self {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set");

        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(&db_url)
            .await
            .expect("Could not connect to the database");

        Repository {
            pool
        }
    }
}

impl FromRequest for Repository {
    type Error = Infallible;
    type Future = Ready<Result<Repository, Infallible>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        ready(Ok(req.app_data::<Self>().expect("Repository was not found").clone()))
    }
}

#[derive(Debug, Display, Error, From)]
pub enum DbError {
    #[display(fmt = "Шуканий об'єкт не знайдено")]
    ItemNotFound,
    #[display(fmt = "Помилка серверу")]
    DbError(sqlx::Error)
}

type ResultDb<T> = Result<T, DbError>;