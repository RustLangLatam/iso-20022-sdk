//
// See http://www.w3.org/2000/09/xmldsig# for more information.
// XML Signature Syntax and Processing Version 2.0
//

use std::collections::HashMap;

use validator::Validate;

::lazy_static::lazy_static! {
    static ref CRYPTO_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref DIGEST_VALUE_TYPE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BASE_64_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ID_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[\i-[:]][\c-[:]]*"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "http://www.w3.org/2000/09/xmldsig#".to_string()
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct CryptoBinary {
    #[validate(regex = "CRYPTO_BINARY_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct Object<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub value: HashMap<String, A>,
    #[serde(rename = "@MimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "@Encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(rename = "@Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct SignatureMethod<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "HMACOutputLength", skip_serializing_if = "Option::is_none")]
    pub hmac_output_length: Option<HmacOutputLengthType>,
    #[validate(length(min = 0, ))]
    #[serde(flatten, default, skip_serializing_if = "HashMap::is_empty")]
    pub value: HashMap<String, A>,
    #[serde(rename = "@Algorithm")]
    pub algorithm: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct Transforms<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Transform", default, skip_serializing_if = "Vec::is_empty")]
    pub transform: Vec<Transform<A>>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct TransformTypeEnum<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "any", skip_serializing_if = "Option::is_none")]
    pub any: Option<A>,
    #[serde(rename = "XPath", skip_serializing_if = "Option::is_none")]
    pub x_path: Option<super::xpath::XPath>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct Transform<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(flatten)]
    #[validate(length(min = 0, ))]
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub value: HashMap<String, TransformTypeEnum<A>>,
    #[serde(rename = "@Algorithm")]
    pub algorithm: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct X509DataTypeEnum<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "X509IssuerSerial", skip_serializing_if = "Option::is_none")]
    pub x_509_issuer_serial: Option<X509IssuerSerial>,
    #[serde(rename = "X509CRL", skip_serializing_if = "Option::is_none")]
    pub x_509_crl: Option<X509Value>,
    #[serde(rename = "any", skip_serializing_if = "Option::is_none")]
    pub any: Option<A>,
    #[serde(rename = "X509SKI", skip_serializing_if = "Option::is_none")]
    pub x_509_ski: Option<X509Value>,
    #[serde(rename = "X509Certificate", skip_serializing_if = "Option::is_none")]
    pub x_509_certificate: Option<X509Value>,
    #[serde(rename = "X509SubjectName", skip_serializing_if = "Option::is_none")]
    pub x_509_subject_name: Option<String>,
}


#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct X509Data<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(flatten)]
    pub value: X509DataTypeEnum<A>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct X509Value {
    #[serde(rename = "$text")]
    pub value: Base64Binary,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct RsaKeyValue {
    #[validate]
    #[serde(rename = "Modulus")]
    pub modulus: CryptoBinary,
    #[validate]
    #[serde(rename = "Exponent")]
    pub exponent: CryptoBinary,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct KeyInfoTypeEnum<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    E: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    F: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "MgmtData", skip_serializing_if = "Option::is_none")]
    pub mgmt_data: Option<MgmtData>,
    #[serde(rename = "RetrievalMethod", skip_serializing_if = "Option::is_none")]
    pub retrieval_method: Option<RetrievalMethod<A>>,
    #[serde(rename = "SPKIData", skip_serializing_if = "Option::is_none")]
    pub spki_data: Option<SpkiData<B>>,
    #[serde(rename = "PGPData", skip_serializing_if = "Option::is_none")]
    pub pgp_data: Option<PgpData<C>>,
    #[serde(rename = "KeyName", skip_serializing_if = "Option::is_none")]
    pub key_name: Option<KeyName>,
    #[serde(rename = "X509Data", skip_serializing_if = "Option::is_none")]
    pub x_509_data: Option<X509Data<D>>,
    #[serde(rename = "any", skip_serializing_if = "Option::is_none")]
    pub any: Option<E>,
    #[serde(rename = "KeyValue", skip_serializing_if = "Option::is_none")]
    pub key_value: Option<KeyValue<F>>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct KeyInfo<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    E: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    F: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(flatten)]
    pub value: KeyInfoTypeEnum<A, B, C, D, E, F>,
    #[serde(rename = "@Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct Manifest<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Reference", default, skip_serializing_if = "Vec::is_empty")]
    pub reference: Vec<Reference<A, B>>,
    #[serde(rename = "@Id")]
    pub id: Option<ID>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct DsaKeyValue {
    #[validate]
    #[serde(rename = "Seed")]
    pub seed: CryptoBinary,
    #[validate]
    #[serde(rename = "PgenCounter")]
    pub pgen_counter: CryptoBinary,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct SignatureValue {
    // #[serde(rename = "SignatureValueType")]
    #[serde(rename = "$text")]
    pub value: Base64Binary,
    #[serde(rename = "@Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct X509IssuerSerial {
    #[serde(rename = "X509IssuerName")]
    pub x_509_issuer_name: String,
    #[serde(rename = "X509SerialNumber")]
    pub x_509_serial_number: i64,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct MgmtData {
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct RetrievalMethod<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Transforms", skip_serializing_if = "Option::is_none")]
    pub transforms: Option<Transforms<A>>,
    #[serde(rename = "@URI", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "@Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct KeyName {
    pub value: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct SignatureProperties<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "SignatureProperty", default, skip_serializing_if = "Vec::is_empty")]
    pub signature_property: Vec<SignatureProperty<A>>,
    #[serde(rename = "@Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct SignedInfo<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "CanonicalizationMethod")]
    pub canonicalization_method: CanonicalizationMethod<A>,
    #[validate]
    #[serde(rename = "SignatureMethod")]
    pub signature_method: SignatureMethod<B>,
    #[serde(rename = "Reference", default, skip_serializing_if = "Vec::is_empty")]
    pub reference: Vec<Reference<C, D>>,
    #[serde(rename = "@Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct DigestMethod<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate(length(min = 0, ))]
    #[serde(flatten, default)]
    pub value: HashMap<String, A>,
    #[serde(rename = "@Algorithm")]
    pub algorithm: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct HmacOutputLengthType {
    #[serde(rename = "$text")]
    pub value: i64,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct Reference<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Transforms", skip_serializing_if = "Option::is_none")]
    pub transforms: Option<Transforms<A>>,
    #[validate]
    #[serde(rename = "DigestMethod")]
    pub digest_method: DigestMethod<B>,
    #[validate]
    #[serde(rename = "DigestValue")]
    pub digest_value: DigestValue,
    #[serde(rename = "@URI", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "@Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "@Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct KeyValueTypeEnum<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "RSAKeyValue", skip_serializing_if = "Option::is_none")]
    pub rsa_key_value: Option<RsaKeyValue>,
    #[serde(rename = "any", skip_serializing_if = "Option::is_none")]
    pub any: Option<A>,
    #[serde(rename = "DSAKeyValue", skip_serializing_if = "Option::is_none")]
    pub dsa_key_value: Option<DsaKeyValue>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct KeyValue<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(flatten)]
    pub value: KeyValueTypeEnum<A>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct Signature<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    E: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    F: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    G: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    H: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    I: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    J: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    K: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "SignedInfo")]
    pub signed_info: SignedInfo<A, B, C, D>,
    #[validate]
    #[serde(rename = "SignatureValue")]
    pub signature_value: SignatureValue,
    #[serde(rename = "KeyInfo", skip_serializing_if = "Option::is_none")]
    pub key_info: Option<KeyInfo<E, F, G, H, I, J>>,
    #[validate(length(min = 0, ))]
    #[serde(rename = "Object", skip_serializing_if = "Vec::is_empty", default)]
    pub object: Vec<Object<K>>,
    #[serde(rename = "@Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct DigestValue {
    #[validate(regex = "DIGEST_VALUE_TYPE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
#[serde(transparent)]
pub struct Base64Binary {
    #[validate(regex = "BASE_64_BINARY_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct PgpData<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "PGPKeyPacket")]
    pub pgp_key_packet: Base64Binary,
    #[validate(length(min = 0, ))]
    #[serde(flatten, default)]
    pub value: HashMap<String, A>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct SpkiData<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(flatten, default)]
    pub value: HashMap<String, A>,
    #[validate]
    #[serde(rename = "SPKISexp")]
    pub spki_sexp: Base64Binary,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct ID {
    #[validate(regex = "ID_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct SignaturePropertyTypeEnum<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "any", skip_serializing_if = "Option::is_none")]
    pub any: Option<A>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct SignatureProperty<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(flatten)]
    #[serde(default)]
    pub value: HashMap<String, SignaturePropertyTypeEnum<A>>,
    #[serde(rename = "@Id")]
    pub id: Option<ID>,
    #[serde(rename = "@Target")]
    pub target: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, validator::Validate)]
pub struct CanonicalizationMethod<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate(length(min = 0, ))]
    #[serde(flatten, default)]
    pub value: HashMap<String, A>,
    #[serde(rename = "@Algorithm")]
    pub algorithm: String,
}

#[cfg(test)]
mod tests {
    use crate::crypto::ecdsa::EcdsaSignature;
    use crate::models::dsig::ecdsa;
    use crate::primitive::Dmkr;
    use crate::utils::XmlExt;

    use super::*;

    #[test]
    fn test_parse_xml_document() {
        let file = r#"<Signature xmlns="http://www.w3.org/2000/09/xmldddddsig#">
            <SignedInfo>
                     <CanonicalizationMethod Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#" />
                     <SignatureMethod Algorithm="http://www.w3.org/2001/04/xmldsig-more#ecdsa-sha256" />
                     <Reference URI="">
                        <Transforms>
                           <Transform Algorithm="http://www.w3.org/2000/09/xmldsig#enveloped-signature" />
                           <Transform Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#" />
                        </Transforms>
                        <DigestMethod Algorithm="http://www.w3.org/2001/04/xmlenc#sha256" />
                        <DigestValue>9A4u9FDDvQS0fcqS76EbS5Ir95wh3JOu2QldyyfWrHs=</DigestValue>
                     </Reference>
                  </SignedInfo>
                  <SignatureValue>AnotherBase64EncodedValue===</SignatureValue>
        </Signature>"#;
        let file =
            std::fs::read_to_string("test/resources/head/head.001.001.01.xml").expect("Unable to read file");

        let doc = EcdsaSignature {
            inner: Signature::<
                Dmkr,
                ecdsa::EcdsaKeyValue<
                    Dmkr,
                    Dmkr,
                    Dmkr,
                    Dmkr,
                    Dmkr,
                    ecdsa::HexBinary,
                    ecdsa::HexBinary,
                >,
                Dmkr,
                Dmkr,
                Dmkr,
                Dmkr,
                Dmkr,
                Dmkr,
                Dmkr,
                Dmkr,
                Dmkr,
            >::default(),
        };

        println!("{:#?}", doc.to_xml());

        // println!("{}", serde_json::to_string_pretty(&doc.value.sgntr).unwrap());

        // assert!(doc.is_ok());
        // assert!(doc.unwrap().validate().is_ok());
        assert!(false)
    }
}
