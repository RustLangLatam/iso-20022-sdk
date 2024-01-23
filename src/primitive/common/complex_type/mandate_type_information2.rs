#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct MandateTypeInformation2 {
    #[serde(rename = "SvcLvl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svc_lvl: Option<super::ServiceLevel8Choice>,
    #[serde(rename = "LclInstrm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcl_instrm: Option<super::LocalInstrument2Choice>,
    #[serde(rename = "CtgyPurp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctgy_purp: Option<super::CategoryPurpose1Choice>,
    #[serde(rename = "Clssfctn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clssfctn: Option<super::MandateClassification1Choice>,
}