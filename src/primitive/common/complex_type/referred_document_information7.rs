#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct ReferredDocumentInformation7 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<super::ReferredDocumentType4>,
    #[serde(rename = "Nb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "RltdDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rltd_dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "LineDtls")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub line_dtls: Vec<super::DocumentLineInformation1>,
}