use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct RemittanceLocationData1 {
    #[serde(rename = "Mtd")]
    pub mtd: super::super::simple_type::RemittanceLocationMethod2Code,
    #[serde(rename = "ElctrncAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elctrnc_adr: Option<super::super::simple_type::Max2048Text>,
    #[serde(rename = "PstlAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<super::NameAndAddress16>,
}