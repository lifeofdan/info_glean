use dirtybase_app::core::AppService;
use dirtybase_db::types::IntoColumnAndValue;

use crate::dirtybase_entry::model::{
    field::{Field, FieldType},
    form::Form,
    section::Section,
};

pub(crate) async fn seed_demo(app: &AppService) {
    seed_forms(&app).await;
    seed_sections(&app).await;
    seed_fields(&app).await;
}

async fn seed_forms(app: &AppService) {
    let forms = match app
        .schema_manger()
        .select_from_table("forms", |builder| {
            builder.eq("name", "Foo form");
        })
        .fetch_all_to::<Form>()
        .await
    {
        Ok(Some(f)) => f,
        _ => Vec::new(),
    };

    dbg!("{:#?}", &forms);

    if forms.is_empty() {
        let form = Form::build_form("Foo form".to_string());

        app.schema_manger().insert("forms", form).await;
    }
}

async fn seed_sections(app: &AppService) {
    let forms = match app
        .schema_manger()
        .select_from_table("forms", |builder| {
            builder.eq("name", "Foo form");
        })
        .fetch_all_to::<Form>()
        .await
    {
        Ok(Some(f)) => f,
        _ => Vec::new(),
    };

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
        for form in forms {
            let section = Section::build_section("Foo Section".to_string(), form.get_id());

            app.schema_manger()
                .insert("sections", section.into_column_value())
                .await;
        }
    }
}

async fn seed_fields(app: &AppService) {
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

    let fields = match app
        .schema_manger()
        .select_from_table("fields", |builder| {
            builder.select_all();
        })
        .fetch_all_to::<Field>()
        .await
    {
        Ok(Some(f)) => f,
        _ => Vec::new(),
    };

    if fields.len() != 2 {
        for section in sections {
            let text_field = Field::build_field(FieldType::Text, section.get_id());
            let textarea_field = Field::build_field(FieldType::TextArea, section.get_id());

            app.schema_manger().insert("fields", text_field).await;
            app.schema_manger().insert("fields", textarea_field).await;
        }
    }
}
