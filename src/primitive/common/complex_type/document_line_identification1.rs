#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct DocumentLineIdentification1 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<super::DocumentLineType1>,
    #[serde(rename = "Nb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "RltdDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rltd_dt: Option<super::super::simple_type::ISODate>,
}