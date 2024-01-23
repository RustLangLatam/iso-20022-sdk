#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct Contact4 {
    #[serde(rename = "NmPrfx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<super::super::simple_type::NamePrefix2Code>,
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<super::super::simple_type::Max140Text>,
    #[serde(rename = "PhneNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<super::super::simple_type::PhoneNumber>,
    #[serde(rename = "MobNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mob_nb: Option<super::super::simple_type::PhoneNumber>,
    #[serde(rename = "FaxNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<super::super::simple_type::PhoneNumber>,
    #[serde(rename = "EmailAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<super::super::simple_type::Max2048Text>,
    #[serde(rename = "EmailPurp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_purp: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "JobTitl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_titl: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "Rspnsblty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rspnsblty: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "Dept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept: Option<super::super::simple_type::Max70Text>,
    #[serde(rename = "Othr")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub othr: Vec<super::OtherContact1>,
    #[serde(rename = "PrefrdMtd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefrd_mtd: Option<super::super::simple_type::PreferredContactMethod1Code>,
}