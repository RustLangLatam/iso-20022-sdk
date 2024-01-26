#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RemittanceLocationMethod2Code {
    FAXI,
    EDIC,
    URID,
    EMAL,
    POST,
    SMSM,
    #[default]
    UNKNOWN,
}

impl RemittanceLocationMethod2Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RemittanceLocationMethod2Code::FAXI => "FAXI",
            RemittanceLocationMethod2Code::EDIC => "EDIC",
            RemittanceLocationMethod2Code::URID => "URID",
            RemittanceLocationMethod2Code::EMAL => "EMAL",
            RemittanceLocationMethod2Code::POST => "POST",
            RemittanceLocationMethod2Code::SMSM => "SMSM",
            _ => "UNKNOWN"
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "FAXI" => Some(RemittanceLocationMethod2Code::FAXI),
            "EDIC" => Some(RemittanceLocationMethod2Code::EDIC),
            "URID" => Some(RemittanceLocationMethod2Code::URID),
            "EMAL" => Some(RemittanceLocationMethod2Code::EMAL),
            "POST" => Some(RemittanceLocationMethod2Code::POST),
            "SMSM" => Some(RemittanceLocationMethod2Code::SMSM),
            _ => None
        }
    }
}