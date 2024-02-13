use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PostalAddress24 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub adr_tp: Option<super::AddressType3Choice>,
    #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dept: Option<super::super::simple_type::Max70Text>,
    #[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub sub_dept: Option<super::super::simple_type::Max70Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub strt_nm: Option<super::super::simple_type::Max70Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub bldg_nb: Option<super::super::simple_type::Max16Text>,
    #[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub bldg_nm: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub flr: Option<super::super::simple_type::Max70Text>,
    #[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pst_bx: Option<super::super::simple_type::Max16Text>,
    #[serde(rename = "Room", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub room: Option<super::super::simple_type::Max70Text>,
    #[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pst_cd: Option<super::super::simple_type::Max16Text>,
    #[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub twn_nm: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub twn_lctn_nm: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dstrct_nm: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ctry_sub_dvsn: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ctry: Option<super::super::simple_type::CountryCode>,
    #[serde(default, rename = "AdrLine", skip_serializing_if = "<[_]>::is_empty")]
    #[validate(length(min = 0, max = 7))]
    pub adr_line: Vec<super::super::simple_type::Max70Text>,
}
