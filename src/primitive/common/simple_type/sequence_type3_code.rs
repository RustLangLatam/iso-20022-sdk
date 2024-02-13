#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum SequenceType3Code {
    FRST,
    RCUR,
    FNAL,
    OOFF,
    RPRE,
    #[default]
    UNKNOWN,
}

impl SequenceType3Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SequenceType3Code::FRST => "FRST",
            SequenceType3Code::RCUR => "RCUR",
            SequenceType3Code::FNAL => "FNAL",
            SequenceType3Code::OOFF => "OOFF",
            SequenceType3Code::RPRE => "RPRE",
            _ => "UNKNOWN",
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "FRST" => Some(SequenceType3Code::FRST),
            "RCUR" => Some(SequenceType3Code::RCUR),
            "FNAL" => Some(SequenceType3Code::FNAL),
            "OOFF" => Some(SequenceType3Code::OOFF),
            "RPRE" => Some(SequenceType3Code::RPRE),
            _ => None,
        }
    }
}
