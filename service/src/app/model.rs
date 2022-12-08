use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub trait Uid {
    fn uid(&self) -> &str;
}

/// Order model used for [Db](super::database::Db) and [Cache](super::cache::Cache) storage
#[derive(sqlx::FromRow)]
pub struct OrderModel {
    uid: String,
    data: sqlx::types::Json<Order>,
}

impl FromStr for OrderModel {
    type Err = serde_json::Error;

    fn from_str(json: &str) -> Result<Self, Self::Err> {
        let order: Order = serde_json::from_str(json)?;
        Ok(OrderModel {
            uid: order.uid().to_owned(),
            data: sqlx::types::Json(order),
        })
    }
}

impl Uid for OrderModel {
    fn uid(&self) -> &str {
        self.uid.as_ref()
    }
}

impl OrderModel {
    pub fn json(&self) -> &sqlx::types::Json<Order> {
        &self.data
    }

    pub fn data(&self) -> &Order {
        &self.json().0
    }
}

/// Json order model
#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Order {
    order_uid: String,
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    locale: String,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: i32,
    date_created: DateTime<Utc>,
    oof_shard: String,
}

impl Uid for Order {
    fn uid(&self) -> &str {
        self.order_uid.as_ref()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Payment {
    transaction: String,
    request_id: String,
    currency: String,
    provider: String,
    amount: i32,
    payment_dt: i32,
    bank: String,
    delivery_cost: i32,
    goods_total: i32,
    custom_fee: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    chrt_id: i32,
    track_number: String,
    price: i32,
    rid: String,
    name: String,
    sale: i32,
    size: String,
    total_price: i32,
    nm_id: i32,
    brand: String,
    status: i32,
}
