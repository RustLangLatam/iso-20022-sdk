use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct SignatureEnvelope<
    Signature: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "$text")]
    pub value: Signature,
}
