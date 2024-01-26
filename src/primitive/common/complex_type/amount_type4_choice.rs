use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct AmountType4Choice {
    #[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub instd_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "EqvtAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub eqvt_amt: Option<super::EquivalentAmount2>,
}