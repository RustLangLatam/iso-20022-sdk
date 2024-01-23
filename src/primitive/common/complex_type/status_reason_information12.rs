#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct StatusReasonInformation12 {
    #[serde(rename = "Orgtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgtr: Option<super::PartyIdentification135>,
    #[serde(rename = "Rsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsn: Option<super::StatusReason6Choice>,
    #[serde(rename = "AddtlInf")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub addtl_inf: Vec<super::super::simple_type::Max105Text>,
}