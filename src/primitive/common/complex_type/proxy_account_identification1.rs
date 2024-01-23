use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct ProxyAccountIdentification1 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<super::ProxyAccountType1Choice>,
    #[serde(rename = "Id")]
    #[validate]
    pub id: super::super::simple_type::Max2048Text,
}