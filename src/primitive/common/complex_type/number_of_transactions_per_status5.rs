use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct NumberOfTransactionsPerStatus5 {
    #[serde(rename = "DtldNbOfTxs")]
    #[validate]
    pub dtld_nb_of_txs: super::super::simple_type::Max15NumericText,
    #[serde(rename = "DtldSts")]
    #[validate]
    pub dtld_sts: super::super::simple_type::ExternalPaymentTransactionStatus1Code,
    #[serde(rename = "DtldCtrlSum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtld_ctrl_sum: Option<super::super::simple_type::DecimalNumber>,
}