use dirtybase_contract::db::macros::DirtyTable;
use dirtybase_db::base::helper::generate_ulid;
use serde::{Deserialize, Serialize};

#[derive(Default, DirtyTable, Debug, Serialize, Deserialize)]
pub struct Form {
    id: String,
    name: String,
}

impl Form {
    pub fn build_form(name: String) -> Form {
        Form {
            id: generate_ulid(),
            name,
        }
    }
}

impl Form {
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }
}
