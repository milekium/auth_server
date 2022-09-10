use std::net::SocketAddr;

use crate::config::DBPool;

mod routes;

pub(super) async fn start(addr: impl Into<SocketAddr>, db_pool: DBPool) {
    warp::serve(routes::make_routes(db_pool)).run(addr).await;
}
