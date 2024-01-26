use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct CreditTransferMandateData1 {
    #[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub mndt_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub tp: Option<super::MandateTypeInformation2>,
    #[serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dt_of_sgntr: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "DtOfVrfctn", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dt_of_vrfctn: Option<super::super::simple_type::ISODateTime>,
    #[serde(rename = "ElctrncSgntr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub elctrnc_sgntr: Option<super::super::simple_type::Max10KBinary>,
    #[serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub frst_pmt_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "FnlPmtDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub fnl_pmt_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "Frqcy", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub frqcy: Option<super::Frequency36Choice>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rsn: Option<super::MandateSetupReason1Choice>,
}