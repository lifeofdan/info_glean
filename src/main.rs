use seed::demo::seed_demo;

mod dirtybase_entry;
mod seed;

#[tokio::main]
async fn main() {
    let app = dirtybase_app::setup().await.unwrap();
    app.register(dirtybase_entry::Extension).await;

    if app.config().environment().is_dev() {
        seed_demo(&app).await;
    }

    let _ = dirtybase_app::run(app).await;
}
