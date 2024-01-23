#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RegulatoryReporting3 {
    #[serde(rename = "DbtCdtRptgInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbt_cdt_rptg_ind: Option<super::super::simple_type::RegulatoryReportingType1Code>,
    #[serde(rename = "Authrty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authrty: Option<super::RegulatoryAuthority2>,
    #[serde(rename = "Dtls")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub dtls: Vec<super::StructuredRegulatoryReporting3>,
}