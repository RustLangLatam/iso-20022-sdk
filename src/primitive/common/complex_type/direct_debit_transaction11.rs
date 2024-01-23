#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct DirectDebitTransaction11 {
    #[serde(rename = "MndtRltdInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mndt_rltd_inf: Option<super::MandateRelatedInformation15>,
    #[serde(rename = "CdtrSchmeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_schme_id: Option<super::PartyIdentification135>,
    #[serde(rename = "PreNtfctnId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_ntfctn_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "PreNtfctnDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_ntfctn_dt: Option<super::super::simple_type::ISODate>,
}