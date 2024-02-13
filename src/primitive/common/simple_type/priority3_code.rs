#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum Priority3Code {
    URGT,
    HIGH,
    NORM,
    #[default]
    UNKNOWN,
}

impl Priority3Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Priority3Code::URGT => "URGT",
            Priority3Code::HIGH => "HIGH",
            Priority3Code::NORM => "NORM",
            _ => "UNKNOWN",
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "URGT" => Some(Priority3Code::URGT),
            "HIGH" => Some(Priority3Code::HIGH),
            "NORM" => Some(Priority3Code::NORM),
            _ => None,
        }
    }
}
