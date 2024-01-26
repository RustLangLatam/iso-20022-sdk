//
// See https://www.ietf.org/rfc/rfc4050.txt for more information
// on ECDSA for XML Digital Signatures
// Copyright (C) The Internet Society (2005).
// Blake-Wilson, et al.

use crate::validator::Validate;

::lazy_static::lazy_static! {
    static ref HEX_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-fA-F]*"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "http://www.w3.org/2001/04/xmldsig-more#".to_string()
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct FieldParamsType<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "$value")]
    pub value: A,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct PrimeFieldParamsType {
    #[validate]
    #[serde(rename = "P")]
    pub p: PositiveInteger,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct ExplicitParamsType<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    E: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "FieldParams")]
    pub field_params: FieldParamsType<A>,
    #[validate]
    #[serde(rename = "CurveParams")]
    pub curve_params: CurveParamsType<B, C>,
    #[validate]
    #[serde(rename = "BasePointParams")]
    pub base_point_params: BasePointParamsType<D, E>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename = "ECDSAKeyValue")]
pub struct EcdsaKeyValueType<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    E: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    F: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    G: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "DomainParameters", skip_serializing_if = "Option::is_none")]
    pub domain_parameters: Option<DomainParamsType<A, B, C, D, E>>,
    #[validate]
    #[serde(rename = "PublicKey")]
    pub public_key: EcPointType<F, G>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct TnBFieldParamsType {
    #[validate]
    #[serde(rename = "K")]
    pub k: PositiveInteger,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct BasePointParamsType<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "BasePoint")]
    pub base_point: EcPointType<A, B>,
    #[validate]
    #[serde(rename = "Order")]
    pub order: PositiveInteger,
    #[serde(rename = "Cofactor", skip_serializing_if = "Option::is_none")]
    pub cofactor: Option<PositiveInteger>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct FieldElemType<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "$value")]
    pub value: A,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct CharTwoFieldElemType {
    #[serde(rename = "@Value")]
    pub value: HexBinary,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct NonNegativeInteger {
    #[validate(range(min = 0, ))]
    #[serde(rename = "$text")]
    pub value: u64,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct PositiveInteger {
    #[validate(range(min = 1, ))]
    #[serde(rename = "$text")]
    pub value: u64,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct DomainParamsTypeEnum<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    E: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "NamedCurve", skip_serializing_if = "Option::is_none")]
    pub named_curve: Option<NamedCurveType>,
    #[serde(rename = "ExplicitParams", skip_serializing_if = "Option::is_none")]
    pub explicit_params: Option<ExplicitParamsType<A, B, C, D, E>>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct DomainParamsType<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    E: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(flatten)]
    pub value: DomainParamsTypeEnum<A, B, C, D, E>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct PrimeFieldElemType {
    #[serde(rename = "@Value")]
    pub value: NonNegativeInteger,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct OddCharExtensionFieldElemType {
    #[serde(rename = "@Value")]
    pub value: NonNegativeInteger,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct OddCharExtensionFieldParamsType {
    #[validate]
    #[serde(rename = "M")]
    pub m: PositiveInteger,
    #[validate]
    #[serde(rename = "W")]
    pub w: PositiveInteger,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct NamedCurveType {
    #[serde(rename = "@URN")]
    pub urn: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct CharTwoFieldParamsType {
    #[validate]
    #[serde(rename = "M")]
    pub m: PositiveInteger,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct PnBFieldParamsType {
    #[validate]
    #[serde(rename = "K1")]
    pub k_1: PositiveInteger,
    #[validate]
    #[serde(rename = "K2")]
    pub k_2: PositiveInteger,
    #[validate]
    #[serde(rename = "K3")]
    pub k_3: PositiveInteger,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct CurveParamsType<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "A")]
    pub a: FieldElemType<A>,
    #[validate]
    #[serde(rename = "B")]
    pub b: FieldElemType<B>,
    #[serde(rename = "Seed", skip_serializing_if = "Option::is_none")]
    pub seed: Option<HexBinary>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct EcdsaKeyValue<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    E: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    F: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    G: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    pub value: EcdsaKeyValueType<A, B, C, D, E, F, G>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct EcPointType<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "X")]
    pub x: FieldElemType<A>,
    #[validate]
    #[serde(rename = "Y")]
    pub y: FieldElemType<B>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct HexBinary {
    #[validate(regex = "HEX_BINARY_REGEX")]
    pub value: String,
}
