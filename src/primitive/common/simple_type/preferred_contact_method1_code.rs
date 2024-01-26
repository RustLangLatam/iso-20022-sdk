#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PreferredContactMethod1Code {
    LETT,
    MAIL,
    PHON,
    FAXX,
    CELL,
    #[default]
    UNKNOWN,
}

impl PreferredContactMethod1Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PreferredContactMethod1Code::LETT => "LETT",
            PreferredContactMethod1Code::MAIL => "MAIL",
            PreferredContactMethod1Code::PHON => "PHON",
            PreferredContactMethod1Code::FAXX => "FAXX",
            PreferredContactMethod1Code::CELL => "CELL",
            _ => "UNKNOWN"
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "LETT" => Some(PreferredContactMethod1Code::LETT),
            "MAIL" => Some(PreferredContactMethod1Code::MAIL),
            "PHON" => Some(PreferredContactMethod1Code::PHON),
            "FAXX" => Some(PreferredContactMethod1Code::FAXX),
            "CELL" => Some(PreferredContactMethod1Code::CELL),
            _ => None
        }
    }
}