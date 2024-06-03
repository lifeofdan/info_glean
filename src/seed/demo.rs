use dirtybase_app::core::AppService;
use dirtybase_db::types::IntoColumnAndValue;

use crate::dirtybase_entry::model::section::Section;

pub(crate) async fn seed_demo(app: &AppService) {
    seed_sections(&app).await;
    seed_inputs(&app).await;
}

async fn seed_sections(app: &AppService) {
    let sections = match app
        .schema_manger()
        .select_from_table("sections", |builder| {
            builder.eq("name", "Foo Section");
        })
        .fetch_all_to::<Section>()
        .await
    {
        Ok(Some(s)) => s,
        _ => Vec::new(),
    };

    if sections.is_empty() {
        let section = Section::build_section("Foo Section".to_string());

        app.schema_manger()
            .insert("sections", section.into_column_value())
            .await;
    }
}

async fn seed_inputs(app: &AppService) {
    let sections = match app
        .schema_manger()
        .select_from_table("sections", |builder| {
            builder.eq("name", "Foo Section");
        })
        .fetch_all_to::<Section>()
        .await
    {
        Ok(Some(s)) => s,
        _ => Vec::new(),
    };

    for section in sections {}
}
