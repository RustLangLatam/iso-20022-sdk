#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum DocumentType3Code {
    RADM,
    RPIN,
    FXDR,
    DISP,
    PUOR,
    SCOR,
    #[default]
    UNKNOWN,
}

impl DocumentType3Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DocumentType3Code::RADM => "RADM",
            DocumentType3Code::RPIN => "RPIN",
            DocumentType3Code::FXDR => "FXDR",
            DocumentType3Code::DISP => "DISP",
            DocumentType3Code::PUOR => "PUOR",
            DocumentType3Code::SCOR => "SCOR",
            _ => "UNKNOWN",
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "RADM" => Some(DocumentType3Code::RADM),
            "RPIN" => Some(DocumentType3Code::RPIN),
            "FXDR" => Some(DocumentType3Code::FXDR),
            "DISP" => Some(DocumentType3Code::DISP),
            "PUOR" => Some(DocumentType3Code::PUOR),
            "SCOR" => Some(DocumentType3Code::SCOR),
            _ => None,
        }
    }
}
