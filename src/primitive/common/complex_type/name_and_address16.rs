use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct NameAndAddress16 {
    #[serde(rename = "Nm")]
    #[validate]
    pub nm: super::super::simple_type::Max140Text,
    #[serde(rename = "Adr")]
    #[validate]
    pub adr: super::PostalAddress24,
}