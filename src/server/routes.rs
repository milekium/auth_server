use std::convert::Infallible;

use crate::config::DBPool;
use crate::errors;
use crate::handlers::health_handler;

use warp::{filters::BoxedFilter, Filter, Reply};

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn make_routes(db_pool: DBPool) -> BoxedFilter<(impl Reply,)> {
    // let health = warp::path::end()
    //     .map(|| StatusCode::OK)
    //     .with(warp::cors().allow_any_origin());

    let health = warp::path!("health")
        .and(with_db(db_pool.clone()))
        .and_then(health_handler);

    let assets = warp::path("assets").and(warp::fs::dir("./src"));

    let user_agent = warp::path("hello")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| format!("Hello {}, whose agent is {}", param, agent));

    health
        .or(user_agent)
        .or(assets)
        .recover(errors::handle_rejection)
        .boxed()
}
