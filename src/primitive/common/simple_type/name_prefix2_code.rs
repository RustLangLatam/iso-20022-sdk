#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum NamePrefix2Code {
    DOCT,
    MADM,
    MISS,
    MIST,
    MIKS,
    #[default]
    UNKNOWN
}

impl NamePrefix2Code {

    pub fn as_str_name(&self) -> &'static str {
        match self {
            NamePrefix2Code::DOCT => "DOCT",
            NamePrefix2Code::MADM => "MADM",
            NamePrefix2Code::MISS => "MISS",
            NamePrefix2Code::MIST => "MIST",
            NamePrefix2Code::MIKS => "MIKS",
            _ => "UNKNOWN"
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "DOCT" => Some(NamePrefix2Code::DOCT),
            "MADM" => Some(NamePrefix2Code::MADM),
            "MISS" => Some(NamePrefix2Code::MISS),
            "MIST" => Some(NamePrefix2Code::MIST),
            "MIKS" => Some(NamePrefix2Code::MIKS),
            _ => None
        }
    }
}