#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ChargeBearerType1Code {
    DEBT,
    CRED,
    SHAR,
    SLEV,
    #[default]
    UNKNOWN,
}

impl ChargeBearerType1Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChargeBearerType1Code::DEBT => "DEBT",
            ChargeBearerType1Code::CRED => "CRED",
            ChargeBearerType1Code::SHAR => "SHAR",
            ChargeBearerType1Code::SLEV => "SLEV",
            _ => "UNKNOWN"
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "DEBT" => Some(ChargeBearerType1Code::DEBT),
            "CRED" => Some(ChargeBearerType1Code::CRED),
            "SHAR" => Some(ChargeBearerType1Code::SHAR),
            "SLEV" => Some(ChargeBearerType1Code::SLEV),
            _ => None
        }
    }
}