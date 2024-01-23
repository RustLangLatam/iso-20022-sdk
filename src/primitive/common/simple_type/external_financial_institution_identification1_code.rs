
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct ExternalFinancialInstitutionIdentification1Code {
    #[validate(length(min = 1, max = 4))]
    #[serde(rename = "$text")]
    pub value: String,
}
