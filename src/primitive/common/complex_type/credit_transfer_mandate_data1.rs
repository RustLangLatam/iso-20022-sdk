#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct CreditTransferMandateData1 {
    #[serde(rename = "MndtId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mndt_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<super::MandateTypeInformation2>,
    #[serde(rename = "DtOfSgntr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt_of_sgntr: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "DtOfVrfctn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt_of_vrfctn: Option<super::super::simple_type::ISODateTime>,
    #[serde(rename = "ElctrncSgntr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elctrnc_sgntr: Option<super::super::simple_type::Max10KBinary>,
    #[serde(rename = "FrstPmtDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frst_pmt_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "FnlPmtDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fnl_pmt_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "Frqcy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frqcy: Option<super::Frequency36Choice>,
    #[serde(rename = "Rsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsn: Option<super::MandateSetupReason1Choice>,
}