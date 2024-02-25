use kraken::tailwindcss;
mod kraken;
use kraken::index;
use axum::{routing::get, Router};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
.route("/styles/tailwind.css", get(tailwindcss::main))
.route("/", get(index::main));

    Ok(router.into())
}
