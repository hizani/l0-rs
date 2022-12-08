use std::collections::HashMap;

use super::{
    database::Db,
    model::{OrderModel, Uid},
};

pub struct Cache(HashMap<String, OrderModel>);

impl Cache {
    pub async fn restore(db: &Db) -> Result<Cache, sqlx::Error> {
        let mut cache = Cache(HashMap::new());
        let data = db.get_orders().await?;
        for model in data {
            cache.0.insert(model.uid().to_owned(), model);
        }
        Ok(cache)
    }

    pub fn add(&mut self, order: OrderModel) {
        self.0.insert(order.uid().to_owned(), order);
    }

    pub fn get_order_by_uid(&self, uid: &str) -> Option<&OrderModel> {
        self.0.get(uid)
    }
}
