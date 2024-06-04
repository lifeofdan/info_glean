use dirtybase_contract::db::macros::DirtyTable;
use dirtybase_db::base::helper::generate_ulid;

#[derive(Default, Debug, Clone)]
pub enum FieldType {
    #[default]
    Text,
    TextArea,
}

#[derive(Default, DirtyTable, Debug, Clone)]
pub struct Field {
    id: String,
    field_type: String,
    sections_id: String,
}

impl ToString for FieldType {
    fn to_string(&self) -> String {
        match self {
            Self::Text => "text".to_string(),
            Self::TextArea => "textarea".to_string(),
        }
    }
}

impl Into<FieldType> for String {
    fn into(self) -> FieldType {
        match self.as_str() {
            "text" => FieldType::Text,
            "textarea" => FieldType::TextArea,
            _ => FieldType::Text,
        }
    }
}

impl Field {
    pub fn build_field(field_type: FieldType, sections_id: String) -> Field {
        Field {
            id: generate_ulid(),
            field_type: field_type.to_string(),
            sections_id,
        }
    }
}
