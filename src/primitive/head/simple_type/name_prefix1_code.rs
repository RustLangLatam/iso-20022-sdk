#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum NamePrefix1Code {
    DOCT,
    MIST,
    MISS,
    MADM,
    #[default]
    UNKNOWN,
}

impl NamePrefix1Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NamePrefix1Code::DOCT => "DOCT",
            NamePrefix1Code::MIST => "MIST",
            NamePrefix1Code::MISS => "MISS",
            NamePrefix1Code::MADM => "MADM",
            _ => "UNKNOWN"
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "DOCT" => Some(NamePrefix1Code::DOCT),
            "MIST" => Some(NamePrefix1Code::MIST),
            "MISS" => Some(NamePrefix1Code::MISS),
            "MADM" => Some(NamePrefix1Code::MADM),
            _ => None
        }
    }
}