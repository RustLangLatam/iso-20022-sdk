use std::io::Write;
use xml::reader::XmlEvent;
use xml::EventReader;

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

    fn to_ident_xml(&self) -> Result<String, quick_xml::DeError> {
        let mut buffer = String::new();
        quick_xml::se::to_writer(&mut buffer, self)?;
        Ok(ident_xml(&buffer))
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
{
}

pub fn ident_xml(buffer: &str) -> String {
    let reader = EventReader::from_str(buffer);
    let mut writer = Vec::new();
    let mut indent_level = 0;
    let mut last_event_was_start = false;
    let mut namespace = String::new();

    for event in reader {
        match event {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                let new_namespace = name.namespace.unwrap_or_default();

                write_indent(&mut writer, indent_level);

                indent_level += 1;
                for attribute in &attributes {
                    last_event_was_characters(&mut writer);
                    writeln!(
                        &mut writer,
                        "<{} {}=\'{}\'>",
                        name.local_name, attribute.name, attribute.value
                    )
                    .unwrap();
                }

                if attributes.is_empty() {
                    if new_namespace != namespace {
                        writeln!(
                            &mut writer,
                            "<{} xmlns=\"{}\">",
                            name.local_name, new_namespace
                        )
                        .unwrap();
                    } else {
                        writeln!(&mut writer, "<{}>", name.local_name).unwrap();
                    }
                }

                namespace = new_namespace;

                last_event_was_start = true;
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                indent_level -= 1;
                if !last_event_was_start {
                    write_indent(&mut writer, indent_level);
                }
                writeln!(&mut writer, "</{}>", name.local_name).unwrap();
                last_event_was_start = false;
            }
            Ok(XmlEvent::Characters(text)) => {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    last_event_was_characters(&mut writer);
                    write!(&mut writer, "{}", trimmed).unwrap();
                }
            }
            _ => {}
        }
    }

    // Removes the last element from a vector \n
    writer.pop();

    String::from_utf8(writer).unwrap()
}

fn write_indent(writer: &mut Vec<u8>, indent_level: usize) {
    let spaces: String = std::iter::repeat(' ').take(indent_level * 4).collect();
    write!(writer, "{}", spaces).unwrap();
}

fn last_event_was_characters(writer: &mut Vec<u8>) {
    if let Some(last_line) = writer.last_mut() {
        let trim_n = String::from_utf8(vec![last_line.clone()]).unwrap();
        if trim_n.ends_with('\n') {
            writer.pop();
        }
    }
}
