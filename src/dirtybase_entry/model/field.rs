use dirtybase_contract::db::macros::DirtyTable;
use dirtybase_db::base::helper::generate_ulid;

#[derive(Debug, Clone)]
enum FieldType {
    TextArea,
    Text,
}

#[derive(Default, DirtyTable, Debug, Clone)]
pub struct Field {
    id: String,
    field_type: FieldType,
}

impl ToString for FieldType {
    fn to_string(&self) -> String {
        match self {
            Self::TextArea => "textarea".to_string(),
            Self::Text => "text".to_string(),
        }
    }
}

impl From<FieldType> for String {
    fn from(value: FieldType) -> Self {
        match value {
            FieldType::Text => "text".to_string(),
            FieldType::TextArea => "textarea".to_string(),
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
    pub fn build_field(field_type: FieldType) -> Field {
        Field {
            id: generate_ulid(),
            field_type,
        }
    }
}
