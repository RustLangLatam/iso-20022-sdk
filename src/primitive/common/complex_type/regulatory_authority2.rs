use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RegulatoryAuthority2 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub nm: Option<super::super::simple_type::Max140Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ctry: Option<super::super::simple_type::CountryCode>,
}
