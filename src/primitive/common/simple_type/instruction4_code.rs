#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Instruction4Code {
    PHOA,
    TELA,
    #[default]
    UNKNOWN,
}

impl Instruction4Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Instruction4Code::PHOA => "PHOA",
            Instruction4Code::TELA => "TELA",
            _ => "UNKNOWN"
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "PHOA" => Some(Instruction4Code::PHOA),
            "TELA" => Some(Instruction4Code::TELA),
            _ => None
        }
    }
}