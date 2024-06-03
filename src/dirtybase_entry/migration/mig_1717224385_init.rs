use dirtybase_contract::db::base::manager::Manager;
use dirtybase_contract::db::migration::Migration;

pub struct Mig1717224385Init;

#[dirtybase_contract::async_trait]
impl Migration for Mig1717224385Init {
    async fn up(&self, manager: &Manager) {
        // Create sections table
        manager
            .create_table_schema("sections", |blueprint| {
                blueprint.id_set();
                blueprint.string("name").set_is_nullable(true);
            })
            .await;

        // Create fields table
        manager
            .create_table_schema("fields", |blueprint| {
                blueprint.id_set();
                blueprint.string("input_type");
                blueprint.ulid_fk("sections", true);
            })
            .await;
        println!("This is a test going up");
    }

    async fn down(&self, manager: &Manager) {
        manager.drop_table("fields").await;
        manager.drop_table("sections").await;

        println!("This is a test going down");
    }
}
