#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum Authorisation1Code {
    AUTH,
    FDET,
    FSUM,
    ILEV,
    #[default]
    UNKNOWN,
}

impl Authorisation1Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Authorisation1Code::AUTH => "AUTH",
            Authorisation1Code::FDET => "FDET",
            Authorisation1Code::FSUM => "FSUM",
            Authorisation1Code::ILEV => "ILEV",
            _ => "UNKNOWN",
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "AUTH" => Some(Authorisation1Code::AUTH),
            "FDET" => Some(Authorisation1Code::FDET),
            "FSUM" => Some(Authorisation1Code::FSUM),
            "ILEV" => Some(Authorisation1Code::ILEV),
            _ => None,
        }
    }
}
