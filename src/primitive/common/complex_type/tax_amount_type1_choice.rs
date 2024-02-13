use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TaxAmountType1Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cd: Option<super::super::simple_type::ExternalTaxAmountType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prtry: Option<super::super::simple_type::Max35Text>,
}
