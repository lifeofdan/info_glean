use dirtybase_contract::db::macros::DirtyTable;
use dirtybase_db::base::helper::generate_ulid;
use serde::{Deserialize, Serialize};

#[derive(Default, DirtyTable, Debug, Serialize, Deserialize)]
pub struct Section {
    id: String,
    name: String,
    forms_id: String,
}

impl Section {
    pub fn build_section(name: String, forms_id: String) -> Section {
        Section {
            id: generate_ulid(),
            name,
            forms_id,
        }
    }
}

impl Section {
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }
}
