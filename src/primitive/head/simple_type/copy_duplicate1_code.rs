#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum CopyDuplicate1Code {
    CODU,
    COPY,
    DUPL,
    #[default]
    UNKNOWN,
}

impl CopyDuplicate1Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CopyDuplicate1Code::CODU => "CODU",
            CopyDuplicate1Code::COPY => "COPY",
            CopyDuplicate1Code::DUPL => "DUPL",
            _ => "UNKNOWN",
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "CODU" => Some(CopyDuplicate1Code::CODU),
            "COPY" => Some(CopyDuplicate1Code::COPY),
            "DUPL" => Some(CopyDuplicate1Code::DUPL),
            _ => None,
        }
    }
}
