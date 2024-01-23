#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct AccountIdentification4Choice {
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<super::super::simple_type::IBAN2007Identifier>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<super::GenericAccountIdentification1>,
}