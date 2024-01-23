#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TaxInformation10 {
    #[serde(rename = "Cdtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<super::TaxParty1>,
    #[serde(rename = "Dbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<super::TaxParty2>,
    #[serde(rename = "AdmstnZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admstn_zone: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "RefNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_nb: Option<super::super::simple_type::Max140Text>,
    #[serde(rename = "Mtd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtd: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "TtlTaxblBaseAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_taxbl_base_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TtlTaxAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_tax_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Dt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "SeqNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<super::super::simple_type::Number>,
    #[serde(rename = "Rcrd")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub rcrd: Vec<super::TaxRecord3>,
}