use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct ReferredDocumentInformation7 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub tp: Option<super::ReferredDocumentType4>,
    #[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub nb: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rltd_dt: Option<super::super::simple_type::ISODate>,
    #[serde(default, rename = "LineDtls", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub line_dtls: Vec<super::DocumentLineInformation1>,
}
