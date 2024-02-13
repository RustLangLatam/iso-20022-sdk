use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TaxParty1 {
    #[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub tax_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub regn_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub tax_tp: Option<super::super::simple_type::Max35Text>,
}
