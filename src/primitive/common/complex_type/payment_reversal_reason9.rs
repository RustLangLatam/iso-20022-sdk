use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PaymentReversalReason9 {
    #[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub orgtr: Option<super::PartyIdentification135>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rsn: Option<super::ReversalReason4Choice>,
    #[serde(default, rename = "AddtlInf", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub addtl_inf: Vec<super::super::simple_type::Max105Text>,
}
