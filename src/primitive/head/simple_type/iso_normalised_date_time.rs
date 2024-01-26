::lazy_static::lazy_static! {
    pub(super) static ref ISO_NORMALISED_DATE_TIME_REGEX: ::regex::Regex = ::regex::Regex::new(r#":*Z\z"#).unwrap();
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct ISONormalisedDateTime {
    #[validate(regex = "ISO_NORMALISED_DATE_TIME_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}

#[cfg(test)]
mod tests {
    use crate::validator::Validate;

    use super::*;

    #[test]
    fn test_parse_xml_document() {
        let doc = ISONormalisedDateTime {
            value: "2018-04-27T20:53:13Z".to_string(),
        };
        assert!(doc.validate().is_ok())
    }
}
