use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct SupplementaryData1<A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate> {
    #[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub plc_and_nm: Option<super::super::simple_type::Max350Text>,
    #[serde(rename = "Envlp")]
    #[validate]
    pub envlp: super::SupplementaryDataEnvelope1<A>,
}