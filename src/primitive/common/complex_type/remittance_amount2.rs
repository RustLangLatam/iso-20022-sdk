#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RemittanceAmount2 {
    #[serde(rename = "DuePyblAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_pybl_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "DscntApldAmt")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub dscnt_apld_amt: Vec<super::DiscountAmountAndType1>,
    #[serde(rename = "CdtNoteAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdt_note_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TaxAmt")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub tax_amt: Vec<super::TaxAmountAndType1>,
    #[serde(rename = "AdjstmntAmtAndRsn")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub adjstmnt_amt_and_rsn: Vec<super::DocumentAdjustment1>,
    #[serde(rename = "RmtdAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmtd_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
}