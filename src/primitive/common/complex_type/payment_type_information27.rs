use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PaymentTypeInformation27 {
    #[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
    pub instr_prty: Option<super::super::simple_type::Priority2Code>,
    #[serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none")]
    pub clr_chanl: Option<super::super::simple_type::ClearingChannel2Code>,
    #[serde(default, rename = "SvcLvl", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub svc_lvl: Vec<super::ServiceLevel8Choice>,
    #[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub lcl_instrm: Option<super::LocalInstrument2Choice>,
    #[serde(rename = "SeqTp", skip_serializing_if = "Option::is_none")]
    pub seq_tp: Option<super::super::simple_type::SequenceType3Code>,
    #[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ctgy_purp: Option<super::CategoryPurpose1Choice>,
}
