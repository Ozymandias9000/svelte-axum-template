use std::any::Any;

use edgedb_tokio::{self, Client};
use tokio::sync::OnceCell;

static POOL: OnceCell<Client> = OnceCell::const_new();

pub async fn create_instance() -> Result<&'static Client, edgedb_tokio::Error> {
    POOL.get_or_try_init(|| async { edgedb_tokio::create_client().await })
        .await
}
