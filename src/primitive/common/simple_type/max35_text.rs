
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct Max35Text {
    #[validate(length(min = 1, max = 35))]
    #[serde(rename = "$text")]
    pub value: String,
}

#[cfg(test)]
mod tests {
    use validator::Validate;
    use super::*;

    #[test]
    fn test_regex() {
        let doc = Max35Text{ value: "WYTTXXXXWYTTXXXXWYTTXXXXWYTTXXXXWYTTXXXXWYTTXXXX".to_string() };
        println!("{:?}", doc.validate());
        assert!(doc.validate().is_ok())
    }
}