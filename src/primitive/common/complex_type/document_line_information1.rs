use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct DocumentLineInformation1 {
    #[serde(rename = "Id")]
    #[validate(length(min = 1,))]
    pub id: Vec<super::DocumentLineIdentification1>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub desc: Option<super::super::simple_type::Max2048Text>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub amt: Option<super::RemittanceAmount3>,
}