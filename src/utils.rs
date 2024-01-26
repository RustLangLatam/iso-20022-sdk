/// A trait for XML serialization and deserialization with validation.
pub trait XmlExt
    where
        Self: Sized + ::serde::Serialize + ::serde::de::DeserializeOwned + ::validator::Validate,
{
    /// Serializes the structure to XML.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the XML string on success, or an error if serialization fails.
    fn to_xml(&self) -> Result<String, quick_xml::DeError> {
        let mut buffer = String::new();
        quick_xml::se::to_writer(&mut buffer, self)?;

        Ok(buffer)
    }

    /// Deserializes the structure from XML and performs validation.
    ///
    /// # Arguments
    ///
    /// * `xml_str` - The XML string to deserialize.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the deserialized structure on success, or an error if
    /// deserialization or validation fails.
    fn from_xml(xml_str: &str) -> Result<Self, quick_xml::DeError> {
        // Deserialize the XML string into the target structure.
        let value: Self = quick_xml::de::from_str(xml_str)?;

        // Validate the deserialized structure.
        value
            .validate()
            .map_err(|err| quick_xml::DeError::Custom(err.to_string()))?;

        Ok(value)
    }
}

impl<T: ?Sized> XmlExt for T where
    T: ::serde::Serialize + serde::de::DeserializeOwned + ::validator::Validate
{}
