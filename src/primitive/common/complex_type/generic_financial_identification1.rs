use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct GenericFinancialIdentification1 {
    #[serde(rename = "Id")]
    #[validate]
    pub id: super::super::simple_type::Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub schme_nm: Option<super::FinancialIdentificationSchemeName1Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub issr: Option<super::super::simple_type::Max35Text>,
}
