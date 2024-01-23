use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct DocumentAdjustment1 {
    #[serde(rename = "Amt")]
    #[validate]
    pub amt: super::ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<super::super::simple_type::CreditDebitCode>,
    #[serde(rename = "Rsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsn: Option<super::super::simple_type::Max4Text>,
    #[serde(rename = "AddtlInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<super::super::simple_type::Max140Text>,
}