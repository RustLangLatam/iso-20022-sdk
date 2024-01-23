#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PostalAddress6 {
    #[serde(rename = "AdrTp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<crate::primitive::AddressType2Code>,
    #[serde(rename = "Dept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept: Option<crate::primitive::Max70Text>,
    #[serde(rename = "SubDept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_dept: Option<crate::primitive::Max70Text>,
    #[serde(rename = "StrtNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<crate::primitive::Max70Text>,
    #[serde(rename = "BldgNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<crate::primitive::Max16Text>,
    #[serde(rename = "PstCd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pst_cd: Option<crate::primitive::Max16Text>,
    #[serde(rename = "TwnNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<crate::primitive::Max35Text>,
    #[serde(rename = "CtrySubDvsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<crate::primitive::Max35Text>,
    #[serde(rename = "Ctry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctry: Option<crate::primitive::CountryCode>,
    #[serde(rename = "AdrLine")]
    #[validate(length(min = 0, max = 7))]
    #[serde(default)]
    pub adr_line: Vec<crate::primitive::Max70Text>,
}