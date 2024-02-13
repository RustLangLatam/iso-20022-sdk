use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct AmendmentInformationDetails14 {
    #[serde(rename = "OrgnlMndtId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_mndt_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "OrgnlCdtrSchmeId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_cdtr_schme_id: Option<super::PartyIdentification135>,
    #[serde(rename = "OrgnlCdtrAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_cdtr_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "OrgnlCdtrAgtAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_cdtr_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "OrgnlDbtr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_dbtr: Option<super::PartyIdentification135>,
    #[serde(rename = "OrgnlDbtrAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_dbtr_acct: Option<super::CashAccount40>,
    #[serde(rename = "OrgnlDbtrAgt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_dbtr_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "OrgnlDbtrAgtAcct", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_dbtr_agt_acct: Option<super::CashAccount40>,
    #[serde(rename = "OrgnlFnlColltnDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_fnl_colltn_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "OrgnlFrqcy", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_frqcy: Option<super::Frequency36Choice>,
    #[serde(rename = "OrgnlRsn", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_rsn: Option<super::MandateSetupReason1Choice>,
    #[serde(rename = "OrgnlTrckgDays", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgnl_trckg_days: Option<super::super::simple_type::Exact2NumericText>,
}
