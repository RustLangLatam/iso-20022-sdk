// See XML-Signature Filter 2.0 schema
// https://www.w3.org/TR/xmldsig-filter2/

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "http://www.w3.org/2002/04/xmldsig-filter2".to_string()
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename = "XPath")]
pub struct XPathType {
    #[serde(rename = "$value")]
    pub value: String,
    #[serde(rename = "@Filter")]
    pub filter: FilterType,
}

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FilterType {
    #[serde(rename = "intersect")]
    Intersect,
    #[serde(rename = "subtract")]
    Subtract,
    #[serde(rename = "union")]
    Union,
    #[default]
    Unknown,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct XPath {
    pub value: XPathType,
}
