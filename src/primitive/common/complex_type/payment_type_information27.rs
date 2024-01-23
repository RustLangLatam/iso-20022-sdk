#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PaymentTypeInformation27 {
    #[serde(rename = "InstrPrty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_prty: Option<super::super::simple_type::Priority2Code>,
    #[serde(rename = "ClrChanl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_chanl: Option<super::super::simple_type::ClearingChannel2Code>,
    #[serde(rename = "SvcLvl")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub svc_lvl: Vec<super::ServiceLevel8Choice>,
    #[serde(rename = "LclInstrm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcl_instrm: Option<super::LocalInstrument2Choice>,
    #[serde(rename = "SeqTp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_tp: Option<super::super::simple_type::SequenceType3Code>,
    #[serde(rename = "CtgyPurp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctgy_purp: Option<super::CategoryPurpose1Choice>,
}