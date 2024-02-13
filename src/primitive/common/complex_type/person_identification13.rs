use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct PersonIdentification13 {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dt_and_plc_of_birth: Option<super::DateAndPlaceOfBirth1>,
    #[serde(default, rename = "Othr", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub othr: Vec<super::GenericPersonIdentification1>,
}
