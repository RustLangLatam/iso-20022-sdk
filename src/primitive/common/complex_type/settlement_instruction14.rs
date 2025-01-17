use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct SettlementInstruction14 {
    #[serde(rename = "SttlmMtd")]
    pub sttlm_mtd: super::super::simple_type::SettlementMethod2Code,
    #[serde(rename = "SttlmAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub sttlm_acct: Option<super::CashAccount40>,
    #[serde(rename = "ClrSys", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub clr_sys: Option<super::ClearingSystemIdentification3Choice>,
}
