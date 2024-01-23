#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TaxParty1 {
    #[serde(rename = "TaxId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "RegnId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regn_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "TaxTp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<super::super::simple_type::Max35Text>,
}