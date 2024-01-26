use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RemittanceLocationData1 {
    #[serde(rename = "Mtd")]
    pub mtd: super::super::simple_type::RemittanceLocationMethod2Code,
    #[serde(rename = "ElctrncAdr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub elctrnc_adr: Option<super::super::simple_type::Max2048Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub pstl_adr: Option<super::NameAndAddress16>,
}