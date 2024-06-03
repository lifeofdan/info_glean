use seed::demo::seed_demo;
use std::env;

mod dirtybase_entry;
mod seed;

#[tokio::main]
async fn main() {
    let app = dirtybase_app::setup().await.unwrap();
    let app_env = env::var("DTY_APP_ENV").expect("Cannot find env val");
    app.register(dirtybase_entry::Extension).await;

    if app_env == "dev".to_string() {
        seed_demo(&app).await;
    }

    let _ = dirtybase_app::run(app).await;
}
