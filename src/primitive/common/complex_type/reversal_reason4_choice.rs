#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct ReversalReason4Choice {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<super::super::simple_type::ExternalReversalReason1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<super::super::simple_type::Max35Text>,
}