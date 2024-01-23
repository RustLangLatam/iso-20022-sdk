#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ClearingChannel2Code {
    RTGS,
    RTNS,
    MPNS,
    BOOK,
    #[default]
    UNKNOWN
}

impl ClearingChannel2Code {

    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClearingChannel2Code::RTGS => "RTGS",
            ClearingChannel2Code::RTNS => "RTNS",
            ClearingChannel2Code::MPNS => "MPNS",
            ClearingChannel2Code::BOOK => "BOOK",
            _ => "UNKNOWN"
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "RTGS" => Some(ClearingChannel2Code::RTGS),
            "RTNS" => Some(ClearingChannel2Code::RTNS),
            "MPNS" => Some(ClearingChannel2Code::MPNS),
            "BOOK" => Some(ClearingChannel2Code::BOOK),
            _ => None
        }
    }
}