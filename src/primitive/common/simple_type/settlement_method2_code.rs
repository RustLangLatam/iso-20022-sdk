#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SettlementMethod2Code {
    INDA,
    INGA,
    CLRG,
    #[default]
    UNKNOWN,
}

impl SettlementMethod2Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SettlementMethod2Code::INDA => "INDA",
            SettlementMethod2Code::INGA => "INGA",
            SettlementMethod2Code::CLRG => "CLRG",
            _ => "UNKNOWN"
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "INDA" => Some(SettlementMethod2Code::INDA),
            "INGA" => Some(SettlementMethod2Code::INGA),
            "CLRG" => Some(SettlementMethod2Code::CLRG),
            _ => None
        }
    }
}