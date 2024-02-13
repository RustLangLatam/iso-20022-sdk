use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct ImplementationSpecification1 {
    #[serde(rename = "Regy")]
    #[validate]
    pub regy: crate::primitive::Max350Text,
    #[serde(rename = "Id")]
    #[validate]
    pub id: crate::primitive::Max2048Text,
}
