#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct Contact4 {
    #[serde(rename = "NmPrfx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<crate::primitive::NamePrefix2Code>,
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<crate::primitive::Max140Text>,
    #[serde(rename = "PhneNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<crate::primitive::PhoneNumber>,
    #[serde(rename = "MobNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mob_nb: Option<crate::primitive::PhoneNumber>,
    #[serde(rename = "FaxNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<crate::primitive::PhoneNumber>,
    #[serde(rename = "EmailAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<crate::primitive::Max2048Text>,
    #[serde(rename = "EmailPurp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_purp: Option<crate::primitive::Max35Text>,
    #[serde(rename = "JobTitl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_titl: Option<crate::primitive::Max35Text>,
    #[serde(rename = "Rspnsblty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rspnsblty: Option<crate::primitive::Max35Text>,
    #[serde(rename = "Dept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept: Option<crate::primitive::Max70Text>,
    #[serde(rename = "Othr")]
    #[validate(length(min = 0, ))]
    #[serde(default)]
    pub othr: Vec<super::OtherContact1>,
    #[serde(rename = "PrefrdMtd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefrd_mtd: Option<crate::primitive::PreferredContactMethod1Code>,
}