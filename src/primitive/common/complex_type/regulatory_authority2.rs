#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RegulatoryAuthority2 {
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<super::super::simple_type::Max140Text>,
    #[serde(rename = "Ctry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctry: Option<super::super::simple_type::CountryCode>,
}