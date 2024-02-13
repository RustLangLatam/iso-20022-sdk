use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct DateAndPlaceOfBirth1 {
    #[serde(rename = "BirthDt")]
    #[validate]
    pub birth_dt: super::super::simple_type::ISODate,
    #[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prvc_of_birth: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "CityOfBirth")]
    #[validate]
    pub city_of_birth: super::super::simple_type::Max35Text,
    #[serde(rename = "CtryOfBirth")]
    #[validate]
    pub ctry_of_birth: super::super::simple_type::CountryCode,
}
