use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TaxInformation10 {
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdtr: Option<super::TaxParty1>,
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr: Option<super::TaxParty2>,
    #[serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub admstn_zone: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ref_nb: Option<super::super::simple_type::Max140Text>,
    #[serde(rename = "Mtd", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub mtd: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ttl_taxbl_base_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ttl_tax_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub seq_nb: Option<super::super::simple_type::Number>,
    #[serde(default, rename = "Rcrd", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub rcrd: Vec<super::TaxRecord3>,
}