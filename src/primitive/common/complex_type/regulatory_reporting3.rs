use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RegulatoryReporting3 {
    #[serde(rename = "DbtCdtRptgInd", skip_serializing_if = "Option::is_none")]
    pub dbt_cdt_rptg_ind: Option<super::super::simple_type::RegulatoryReportingType1Code>,
    #[serde(rename = "Authrty", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub authrty: Option<super::RegulatoryAuthority2>,
    #[serde(default, rename = "Dtls", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub dtls: Vec<super::StructuredRegulatoryReporting3>,
}
