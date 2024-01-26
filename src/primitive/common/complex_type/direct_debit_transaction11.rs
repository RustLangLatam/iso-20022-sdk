use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct DirectDebitTransaction11 {
    #[serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub mndt_rltd_inf: Option<super::MandateRelatedInformation15>,
    #[serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdtr_schme_id: Option<super::PartyIdentification135>,
    #[serde(rename = "PreNtfctnId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pre_ntfctn_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "PreNtfctnDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pre_ntfctn_dt: Option<super::super::simple_type::ISODate>,
}