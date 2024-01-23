#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct MandateRelatedInformation15 {
    #[serde(rename = "MndtId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mndt_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "DtOfSgntr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt_of_sgntr: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "AmdmntInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amdmnt_ind: Option<super::super::simple_type::TrueFalseIndicator>,
    #[serde(rename = "AmdmntInfDtls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amdmnt_inf_dtls: Option<super::AmendmentInformationDetails14>,
    #[serde(rename = "ElctrncSgntr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elctrnc_sgntr: Option<super::super::simple_type::Max1025Text>,
    #[serde(rename = "FrstColltnDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frst_colltn_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "FnlColltnDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fnl_colltn_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "Frqcy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frqcy: Option<super::Frequency36Choice>,
    #[serde(rename = "Rsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsn: Option<super::MandateSetupReason1Choice>,
    #[serde(rename = "TrckgDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trckg_days: Option<super::super::simple_type::Exact2NumericText>,
}