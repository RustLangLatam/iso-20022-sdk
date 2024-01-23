#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TaxAuthorisation1 {
    #[serde(rename = "Titl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titl: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<super::super::simple_type::Max140Text>,
}