use dirtybase_contract::db::macros::DirtyTable;
use dirtybase_db::base::helper::generate_ulid;

#[derive(Default, DirtyTable, Debug)]
pub struct Section {
    id: String,
    name: String,
}

impl Section {
    pub fn build_section(name: String) -> Section {
        Section {
            id: generate_ulid(),
            name,
        }
    }
}

impl Section {
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }
}
