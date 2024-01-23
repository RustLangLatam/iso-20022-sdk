#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SettlementMethod1Code {
    INDA,
    INGA,
    COVE,
    CLRG,
    #[default]
    UNKNOWN
}

impl SettlementMethod1Code {

    pub fn as_str_name(&self) -> &'static str {
        match self {
            SettlementMethod1Code::INDA => "INDA",
            SettlementMethod1Code::INGA => "INGA",
            SettlementMethod1Code::COVE => "COVE",
            SettlementMethod1Code::CLRG => "CLRG",
            _ => "UNKNOWN"
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "INDA" => Some(SettlementMethod1Code::INDA),
            "INGA" => Some(SettlementMethod1Code::INGA),
            "COVE" => Some(SettlementMethod1Code::COVE),
            "CLRG" => Some(SettlementMethod1Code::CLRG),
            _ => None
        }
    }
}