use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct SupplementaryDataEnvelope1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(flatten)]
    pub value: A,
}
