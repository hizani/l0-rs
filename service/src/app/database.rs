use sqlx::{postgres::PgPoolOptions, query, query_as, Pool, Postgres};

use super::model::{OrderModel, Uid};
use anyhow::Result;

#[derive(Clone)]
pub struct Db(Pool<Postgres>);

impl Db {
    pub async fn connect(con_string: &str) -> Result<Db> {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(con_string)
            .await?;
        Ok(Db(pool))
    }

    pub async fn insert_order(&self, order: &OrderModel) -> Result<(), sqlx::Error> {
        query(r#"INSERT INTO ORDERS (uid, data) VALUES ($1, $2)"#)
            .bind(order.uid())
            .bind::<_>(order.json())
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub async fn get_orders(&self) -> Result<Vec<OrderModel>, sqlx::Error> {
        let rows = query_as(r#"SELECT * FROM orders"#)
            .fetch_all(&self.0)
            .await?;
        Ok(rows)
    }
}
