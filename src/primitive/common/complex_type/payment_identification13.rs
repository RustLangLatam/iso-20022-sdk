use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PaymentIdentification13 {
    #[serde(rename = "InstrId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "EndToEndId")]
    #[validate]
    pub end_to_end_id: super::super::simple_type::Max35Text,
    #[serde(rename = "TxId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "UETR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uetr: Option<super::super::simple_type::UUIDv4Identifier>,
    #[serde(rename = "ClrSysRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_sys_ref: Option<super::super::simple_type::Max35Text>,
}