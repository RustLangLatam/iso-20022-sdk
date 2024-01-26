use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct DateAndDateTime2Choice {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dt_tm: Option<super::super::simple_type::ISODateTime>,
}