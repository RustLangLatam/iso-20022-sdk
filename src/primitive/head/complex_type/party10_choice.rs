use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct Party10Choice {
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub org_id: Option<super::OrganisationIdentification7>,
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prvt_id: Option<super::PersonIdentification5>,
}
