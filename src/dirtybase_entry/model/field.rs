use dirtybase_contract::db::macros::DirtyTable;
use dirtybase_db::{
    base::helper::generate_ulid,
    field_values::FieldValue,
    types::{ColumnAndValue, FromColumnAndValue},
};

#[derive(Debug, Clone)]
enum FieldType {
    Text,
    TextArea,
}

#[derive(Default, DirtyTable, Debug, Clone)]
pub struct Field {
    id: String,
    field_type: FieldType,
}

impl From<FieldType> for FieldValue {
    fn from(value: FieldType) -> Self {
        Self::String(value.to_string())
    }
}

impl From<&FieldType> for FieldValue {
    fn from(value: &FieldType) -> Self {
        Self::from(value.clone())
    }
}

impl From<FieldValue> for FieldType {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::String(v) => v.into(),
            _ => panic!("value in data base is not a valid FieldType"),
        }
    }
}

impl FromColumnAndValue for FieldType {
    fn from_column_value(column_and_value: ColumnAndValue) -> Self {
        column_and_value.get("type_name").unwrap().clone().into()
    }
}

impl Default for FieldType {
    fn default() -> Self {
        FieldType::Text
    }
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
            v @ _ => panic!("'{}' is not a valid field type", v),
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
