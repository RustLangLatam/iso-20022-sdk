#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CreditDebitCode {
    CRDT,
    DBIT,
    #[default]
    UNKNOWN,
}

impl CreditDebitCode {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CreditDebitCode::CRDT => "CRDT",
            CreditDebitCode::DBIT => "DBIT",
            _ => "UNKNOWN"
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "CRDT" => Some(CreditDebitCode::CRDT),
            "DBIT" => Some(CreditDebitCode::DBIT),
            _ => None
        }
    }
}