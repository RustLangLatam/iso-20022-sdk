use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RemittanceAmount2 {
    #[serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub due_pybl_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(default, rename = "DscntApldAmt", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub dscnt_apld_amt: Vec<super::DiscountAmountAndType1>,
    #[serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdt_note_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(default, rename = "TaxAmt", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub tax_amt: Vec<super::TaxAmountAndType1>,
    #[serde(default, rename = "AdjstmntAmtAndRsn", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub adjstmnt_amt_and_rsn: Vec<super::DocumentAdjustment1>,
    #[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rmtd_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
}