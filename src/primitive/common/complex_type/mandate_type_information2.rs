use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct MandateTypeInformation2 {
    #[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub svc_lvl: Option<super::ServiceLevel8Choice>,
    #[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub lcl_instrm: Option<super::LocalInstrument2Choice>,
    #[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ctgy_purp: Option<super::CategoryPurpose1Choice>,
    #[serde(rename = "Clssfctn", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub clssfctn: Option<super::MandateClassification1Choice>,
}