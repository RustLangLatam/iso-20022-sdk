// Copyright 2023 Emergent Financial, LLC - All Rights Reserved
//
//
// This software is licensed under the Emergent Financial Limited Public License Version 1.0
// (EF-LPLv1). You may use, copy, modify, and distribute this software under the terms and
// conditions of the EF-LPL. For more information, please refer to the full text of the license
// at https://github.com/emergentfinancial/ef-lpl.
//
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

// use iso_20022_dsig::dsig::{Signature, SignatureBuilder};
// use iso_20022_head::head_001_001_03::*;
// use iso_20022_nvlp::nvlp_001_001_01::*;
// use p256::ecdsa::{signature::Signer, Signature as P256Signature, SigningKey};
// use quick_xml::de::from_str;
// use quick_xml::se::to_string;

use std::collections::HashMap;

pub use const_oid::db::rfc5912::SECP_256_R_1;
use elliptic_curve::sec1::{Coordinates, ToEncodedPoint};
use sha2::{Digest, Sha256};

use crate::models::dsig::{dsig, ecdsa, xpath, EcPointType, FieldElemType, ID};
use crate::primitive::Dmkr;

use super::{MessageSigner, XmlSignature};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
// #[serde(transparent)]
pub struct EcdsaSignature {
    #[serde(rename = "Signature")]
    pub(crate) inner: dsig::Signature<
        Dmkr,
        ecdsa::EcdsaKeyValue<Dmkr, Dmkr, Dmkr, Dmkr, Dmkr, ecdsa::HexBinary, ecdsa::HexBinary>,
        Dmkr,
        Dmkr,
        Dmkr,
        Dmkr,
        Dmkr,
        Dmkr,
        Dmkr,
        Dmkr,
        Dmkr,
    >,
}

impl XmlSignature for EcdsaSignature {
    type PublicKey = p256::PublicKey;

    fn set_signed_info(
        self,
        uri: String,
        x_path_transformations: Vec<xpath::XPath>,
        digest_value: dsig::DigestValue,
        public_key: Self::PublicKey,
    ) -> Self {
        let mut sig = self.inner;

        let transform = x_path_transformations
            .into_iter()
            .map(|path| dsig::Transform {
                // use filter2 for xpath transformations
                algorithm: "http://www.w3.org/2002/06/xmldsig-filter2".into(),
                value: HashMap::from([(
                    "".to_string(),
                    dsig::TransformTypeEnum {
                        x_path: Some(path),
                        any: None,
                    },
                )]),
            })
            .collect();

        let id = ID {
            value: uuid::Uuid::new_v4().to_string(),
        };

        let ec_points = match public_key.to_encoded_point(false).coordinates() {
            Coordinates::Uncompressed { x, y } => ecdsa::EcPointType {
                x: FieldElemType {
                    value: ecdsa::HexBinary {
                        value: hex::encode(x),
                    },
                },
                y: FieldElemType {
                    value: ecdsa::HexBinary {
                        value: hex::encode(y),
                    },
                },
            },
            _ => EcPointType::default(),
        };

        sig.signed_info = dsig::SignedInfo {
            id: Some(id.clone()),
            reference: vec![dsig::Reference {
                id: Some(id.clone()),
                r#type: Some("application/xml".to_string()),
                uri: Some(uri),
                // Transforms specify how the document was processed
                // prior to calculating the digest.
                transforms: Some(dsig::Transforms { transform }),
                digest_method: dsig::DigestMethod {
                    algorithm: super::DEFAULT_DIGEST_METHOD_ALGORITHM.into(),
                    value: Default::default(),
                },
                digest_value,
            }],
            canonicalization_method: dsig::CanonicalizationMethod {
                value: Default::default(),
                algorithm: super::DEFAULT_CANONICALIZATION_METHOD_ALGORITHM.into(),
            },
            signature_method: dsig::SignatureMethod {
                // Default bytes for SHA256 hash output
                hmac_output_length: Some(dsig::HmacOutputLengthType {
                    value: super::SHA_256_HMAC_OUTPUT_LENGTH,
                }),
                algorithm: format!("{}#ecdsa-sha256", super::DEFAULT_SIGNATURE_METHOD_ALGORITHM),
                value: HashMap::from([(
                    "".to_string(),
                    ecdsa::EcdsaKeyValue {
                        value: ecdsa::EcdsaKeyValueType {
                            domain_parameters: Some(ecdsa::DomainParamsType {
                                value: ecdsa::DomainParamsTypeEnum {
                                    named_curve: Some(ecdsa::NamedCurveType {
                                        urn: SECP_256_R_1.to_string(),
                                    }),
                                    explicit_params: None,
                                },
                            }),
                            public_key: ec_points,
                            xmlns: ecdsa::namespace(),
                        },
                    },
                )]),
            },
        };

        Self { inner: sig }
    }
}

impl signature::Signer<EcdsaSignature> for MessageSigner<EcdsaSignature> {
    fn try_sign(&self, msg: &[u8]) -> Result<EcdsaSignature, signature::Error> {
        // Hash the message
        let mut hasher = Sha256::new();
        hasher.update(msg);
        let _hash = hasher.finalize();

        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_p256() {
        // let sig = SignatureBuilder::default();

        // let sig = sig.build().ok();

        // println!("sig: {:#?}", sig);
        // println!("sig: {:?}", to_string(&sig));
    }
}
