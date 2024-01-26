use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct GarnishmentType1 {
    #[serde(rename = "CdOrPrtry")]
    #[validate]
    pub cd_or_prtry: super::GarnishmentType1Choice,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub issr: Option<super::super::simple_type::Max35Text>,
}