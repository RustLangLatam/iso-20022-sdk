#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Frequency6Code {
    YEAR,
    MNTH,
    QURT,
    MIAN,
    WEEK,
    DAIL,
    ADHO,
    INDA,
    FRTN,
    #[default]
    UNKNOWN
}

impl Frequency6Code {

    pub fn as_str_name(&self) -> &'static str {
        match self {
            Frequency6Code::YEAR => "YEAR",
            Frequency6Code::MNTH => "MNTH",
            Frequency6Code::QURT => "QURT",
            Frequency6Code::MIAN => "MIAN",
            Frequency6Code::WEEK => "WEEK",
            Frequency6Code::DAIL => "DAIL",
            Frequency6Code::ADHO => "ADHO",
            Frequency6Code::INDA => "INDA",
            Frequency6Code::FRTN => "FRTN",
            _ => "UNKNOWN"
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "YEAR" => Some(Frequency6Code::YEAR),
            "MNTH" => Some(Frequency6Code::MNTH),
            "QURT" => Some(Frequency6Code::QURT),
            "MIAN" => Some(Frequency6Code::MIAN),
            "WEEK" => Some(Frequency6Code::WEEK),
            "DAIL" => Some(Frequency6Code::DAIL),
            "ADHO" => Some(Frequency6Code::ADHO),
            "INDA" => Some(Frequency6Code::INDA),
            "FRTN" => Some(Frequency6Code::FRTN),
            _ => None
        }
    }
}