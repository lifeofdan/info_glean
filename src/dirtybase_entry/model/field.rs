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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_field_type() {
        let field_type_text: FieldType = "text".to_string().into();
        let field_type_textarea: FieldType = "textarea".to_string().into();
        assert_eq!(
            "text",
            field_type_text.to_string(),
            "text should return a Text of the FieldType"
        );
        assert_eq!(
            "textarea",
            field_type_textarea.to_string(),
            "textarea should return a TextArea of the FieldType"
        );
    }

    #[test]
    fn tes_from_field_value_for_field_type() {
        let value = FieldValue::String("text".to_string());
        let field_type_text: FieldType = value.into();

        let value = FieldValue::String("textarea".to_string());
        let field_type_textarea: FieldType = value.into();

        assert_eq!(
            "text",
            field_type_text.to_string(),
            "text should return a Text of the FieldType"
        );
        assert_eq!(
            "textarea",
            field_type_textarea.to_string(),
            "textarea should return a Text of the FieldType"
        );
    }

    #[test]
    fn test_field_type_into_field_value() {
        let text = FieldType::Text;
        let value: FieldValue = text.into();

        assert_eq!(&value.to_string(), "text");
    }
}
