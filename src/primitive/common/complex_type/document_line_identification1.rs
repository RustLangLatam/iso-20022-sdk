use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct DocumentLineIdentification1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub tp: Option<super::DocumentLineType1>,
    #[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub nb: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rltd_dt: Option<super::super::simple_type::ISODate>,
}