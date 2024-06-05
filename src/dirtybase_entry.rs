mod event;
mod event_handler;
mod http;
mod migration;
pub mod model;

use dirtybase_app::{
    axum::{response::Html, Json},
    core::{AppService, AppServiceExtractor},
};
use dirtybase_contract::{axum, dirtybase_config::DirtyConfig, http::RouterManager};

use self::model::form::Form;

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
                .get("/bar", || async { "hi" }, "bar")
        });

        manager.api(Some("/v1"), |router| {
            router.get(
                "/forms",
                |app_ext: AppServiceExtractor| async {
                    let manager = app_ext.inner().schema_manger();
                    let forms = match manager
                        .select_from_table("forms", |builder| {
                            builder.select_all().left_join(
                                "sections",
                                "section.forms_id",
                                "=",
                                "forms.id",
                            );
                        })
                        .fetch_all_to::<Form>()
                        .await
                    {
                        Ok(Some(f)) => f,
                        _ => Vec::new(),
                    };

                    dbg!("{:#?}", &forms);
                    Json(forms)
                },
                "forms",
            )
        });

        manager
    }
}
