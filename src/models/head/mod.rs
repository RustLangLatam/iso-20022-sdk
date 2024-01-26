//! # Head Module
//!
//! The `head` module provides functionality related to the ISO 20022 Business Application Header (BAH).
//! The Business Application Header is a key component in ISO 20022 messages, containing essential
//! information about the sending and receiving systems, message type, and other relevant details.
//!

use crate::primitive::Xmlns;

pub mod head_001_001_01;
pub mod head_001_001_02;
#[cfg(test)]
mod tests;

#[derive(
Debug, Default, Clone, PartialEq, Serialize, Deserialize, Display, EnumIter, EnumAsInner,
)]
pub enum AppHdr<
    Signature: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    // pacs
    head_001_001_01(head_001_001_01::AppHdr<Signature>),
    head_001_001_02(head_001_001_02::AppHdr<Signature>),
    #[default]
    Unknown,
}

impl<
    Signature: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> AppHdr<Signature>
    where
        Signature: ::serde::de::DeserializeOwned,
{
    fn from_xml(s: &str) -> Result<Self, quick_xml::DeError> {
        use crate::utils::XmlExt;
        let xmlns: Xmlns = quick_xml::de::from_str(s)?;
        let schema = xmlns.replace(crate::documents::DEFAULT_XLMNS_PREFIX, "");

        let doc = match schema.as_str() {
            "head.001.001.01" => AppHdr::head_001_001_01(XmlExt::from_xml(s)?),
            "head.001.001.02" => AppHdr::head_001_001_02(XmlExt::from_xml(s)?),
            _ => return Err(quick_xml::DeError::Custom(schema)),
        };

        Ok(doc)
    }
}
