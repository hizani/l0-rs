use anyhow::Result;
use futures_util::StreamExt;
use redis::Client;
use std::{convert::Infallible, net::SocketAddr, str::FromStr, sync::Arc};
use tokio::sync::RwLock;
use warp::{hyper::StatusCode, Filter};

use crate::{app::model::OrderModel, config::Config};

use self::{cache::Cache, database::Db, model::Order};

mod cache;
mod database;
mod model;

pub struct App {
    cache: Arc<RwLock<Cache>>,
    db: Db,
    redis_config: [String; 2],
    server_socket: SocketAddr,
}

impl App {
    pub async fn new(cfg: Config) -> Result<App> {
        // Connect to database
        let conn_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            cfg.database().user(),
            cfg.database().pass(),
            cfg.database().host(),
            cfg.database().port(),
            cfg.database().db()
        );
        let db = Db::connect(&conn_string).await?;

        // Create cache
        let cache = {
            let c = Cache::restore(&db).await?;
            Arc::new(RwLock::new(c))
        };

        // Create server socket
        let server_socket = {
            let conn_string = format!("{}:{}", cfg.server().host(), cfg.server().port());
            SocketAddr::from_str(&conn_string)?
        };

        let redis_config = {
            let conn_string = format!("redis://{}/", cfg.redis().host());
            let channel = cfg.redis().channel().to_owned();
            [conn_string, channel]
        };
        Ok(App {
            cache,
            db,
            redis_config,
            server_socket,
        })
    }

    pub async fn run(self) -> Result<()> {
        // Start server
        let filter = {
            let cache = self.cache.clone();
            warp::path!("order" / String)
                .and(warp::get())
                .and(warp::any().map(move || cache.clone()))
                .and_then(handle_get_by_uid)
                .recover(handle_rejection)
        };
        tokio::try_join!(
            listen_redis_chan(self.redis_config, self.db, self.cache),
            async move {
                println!("{}", self.server_socket);
                warp::serve(filter).run(self.server_socket).await;
                let result: Result<()> = Err(anyhow::anyhow!("server closed"));
                result
            }
        )?;
        unreachable!()
    }
}

async fn listen_redis_chan(cfg: [String; 2], db: Db, cache: Arc<RwLock<Cache>>) -> Result<()> {
    // Connect and subscribe to redis channel
    let client = Client::open(cfg[0].to_owned())?;
    let mut redis_connection = client.get_async_connection().await?.into_pubsub();
    redis_connection.subscribe(&cfg[1]).await?;

    // Start listening channel
    let mut stream = redis_connection.on_message();
    while let Some(message) = stream.next().await {
        let mut cache_lock = cache.write().await;
        let json: String = match message.get_payload() {
            Ok(json) => json,
            Err(e) => {
                println!("Failed getting payload: {e}");
                continue;
            }
        };

        let order_model = match OrderModel::from_str(&json) {
            Ok(o) => o,
            Err(e) => {
                println!("Failed parsing JSON message: {e}");
                continue;
            }
        };
        if let Err(e) = db.insert_order(&order_model).await {
            println!("Failed to insert data into database: {e}");
            continue;
        }
        cache_lock.add(order_model);
    }
    Err(anyhow::anyhow!("redis connection closed"))
}

async fn handle_rejection(_: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::with_status("NOT FOUND", StatusCode::NOT_FOUND))
}

async fn handle_get_by_uid(
    uid: String,
    cache: Arc<RwLock<Cache>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let cache_lock = cache.read().await;
    if let Some(order_model) = cache_lock.get_order_by_uid(&uid) {
        let order: &Order = order_model.into();
        return Ok(warp::reply::html(format!(
            "<html><h1>{}</h1><p>{:#?}</p></html>",
            uid, order
        )));
    }

    Err(warp::reject::not_found())
}
