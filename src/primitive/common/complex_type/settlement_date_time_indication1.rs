use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct SettlementDateTimeIndication1 {
    #[serde(rename = "DbtDtTm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbt_dt_tm: Option<super::super::simple_type::ISODateTime>,
    #[serde(rename = "CdtDtTm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdt_dt_tm: Option<super::super::simple_type::ISODateTime>,
}