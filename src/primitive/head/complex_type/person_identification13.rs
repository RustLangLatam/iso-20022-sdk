#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PersonIdentification13 {
    #[serde(rename = "DtAndPlcOfBirth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<crate::primitive::DateAndPlaceOfBirth1>,
    #[serde(rename = "Othr")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub othr: Vec<super::GenericPersonIdentification1>,
}