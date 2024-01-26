use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct MandateRelatedInformation15 {
    #[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub mndt_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dt_of_sgntr: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "AmdmntInd", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub amdmnt_ind: Option<super::super::simple_type::TrueFalseIndicator>,
    #[serde(rename = "AmdmntInfDtls", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub amdmnt_inf_dtls: Option<super::AmendmentInformationDetails14>,
    #[serde(rename = "ElctrncSgntr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub elctrnc_sgntr: Option<super::super::simple_type::Max1025Text>,
    #[serde(rename = "FrstColltnDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub frst_colltn_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "FnlColltnDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub fnl_colltn_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "Frqcy", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub frqcy: Option<super::Frequency36Choice>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rsn: Option<super::MandateSetupReason1Choice>,
    #[serde(rename = "TrckgDays", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub trckg_days: Option<super::super::simple_type::Exact2NumericText>,
}