use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct Contact4 {
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<crate::primitive::NamePrefix2Code>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub nm: Option<crate::primitive::Max140Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub phne_nb: Option<crate::primitive::PhoneNumber>,
    #[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub mob_nb: Option<crate::primitive::PhoneNumber>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub fax_nb: Option<crate::primitive::PhoneNumber>,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub email_adr: Option<crate::primitive::Max2048Text>,
    #[serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub email_purp: Option<crate::primitive::Max35Text>,
    #[serde(rename = "JobTitl", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub job_titl: Option<crate::primitive::Max35Text>,
    #[serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rspnsblty: Option<crate::primitive::Max35Text>,
    #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dept: Option<crate::primitive::Max70Text>,
    #[serde(default, rename = "Othr", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub othr: Vec<super::OtherContact1>,
    #[serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none")]
    pub prefrd_mtd: Option<crate::primitive::PreferredContactMethod1Code>,
}
