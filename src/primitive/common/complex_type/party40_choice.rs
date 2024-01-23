#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct Party40Choice {
    #[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
    pub pty: Option<super::PartyIdentification135>,
    #[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
    pub agt: Option<super::BranchAndFinancialInstitutionIdentification6>,
}