use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct Charges7 {
    #[serde(rename = "Amt")]
    #[validate]
    pub amt: super::ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "Agt")]
    #[validate]
    pub agt: super::BranchAndFinancialInstitutionIdentification6,
}
