#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum MandateClassification1Code {
    FIXE,
    USGB,
    VARI,
    #[default]
    UNKNOWN,
}

impl MandateClassification1Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MandateClassification1Code::FIXE => "FIXE",
            MandateClassification1Code::USGB => "USGB",
            MandateClassification1Code::VARI => "VARI",
            _ => "UNKNOWN",
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "FIXE" => Some(MandateClassification1Code::FIXE),
            "USGB" => Some(MandateClassification1Code::USGB),
            "VARI" => Some(MandateClassification1Code::VARI),
            _ => None,
        }
    }
}
