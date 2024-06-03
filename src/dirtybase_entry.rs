mod event;
mod event_handler;
mod http;
mod migration;
pub mod model;

use dirtybase_app::axum::response::Html;
use dirtybase_contract::{axum, dirtybase_config::DirtyConfig, http::RouterManager};

pub struct Extension;

#[dirtybase_contract::async_trait]
impl dirtybase_contract::ExtensionSetup for Extension {
    async fn setup(&self, _config: &DirtyConfig) {
        event_handler::setup().await;
    }

    fn migrations(&self) -> dirtybase_contract::ExtensionMigrations {
        migration::setup()
    }

    fn register_routes(&self, mut manager: RouterManager) -> RouterManager {
        // manager.general("/be-awsome",
        //     axum::Router::new().route("/", axum::routing::get(|| async { "Rust or bust" })),
        // );

        manager.general(None, |router| {
            router
                .get("/foo", || async { Html("hello from foo") }, "foo")
                .get("/bar", || async { Html("<h1>Bar</h1>") }, "bar")
        });

        manager
    }
}
