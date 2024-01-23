#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct SettlementDateTimeIndication1 {
    #[serde(rename = "DbtDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbt_dt_tm: Option<super::super::simple_type::ISODateTime>,
    #[serde(rename = "CdtDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdt_dt_tm: Option<super::super::simple_type::ISODateTime>,
}