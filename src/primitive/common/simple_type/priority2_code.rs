#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority2Code {
    HIGH,
    NORM,
    #[default]
    UNKNOWN,
}

impl Priority2Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Priority2Code::HIGH => "HIGH",
            Priority2Code::NORM => "NORM",
            _ => "UNKNOWN"
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "HIGH" => Some(Priority2Code::HIGH),
            "NORM" => Some(Priority2Code::NORM),
            _ => None
        }
    }
}