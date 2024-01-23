use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct GroupHeader101 {
    #[serde(rename = "MsgId")]
    #[validate]
    pub msg_id: super::super::simple_type::Max35Text,
    #[serde(rename = "CreDtTm")]
    #[validate]
    pub cre_dt_tm: super::super::simple_type::ISODateTime,
    #[serde(rename = "InstgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "OrgnlBizQry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_biz_qry: Option<super::OriginalBusinessQuery1>,
}