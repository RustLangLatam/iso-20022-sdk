use serde_json::Value;

pub use self::complex_type::*;
pub use self::simple_type::*;
pub use self::xmlns::Xmlns;

mod complex_type;
mod simple_type;
mod xmlns;

/// Document Marker (Dmkr) is a type that is used as a default value
/// for `any` element types. It effectively is a default document type.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Builder, Validate)]
#[serde(transparent)]
pub struct Dmkr {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}
