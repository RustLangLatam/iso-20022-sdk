#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AddressType2Code {
    ADDR,
    PBOX,
    HOME,
    BIZZ,
    MLTO,
    DLVY,
    #[default]
    UNKNOWN,
}

impl AddressType2Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AddressType2Code::ADDR => "ADDR",
            AddressType2Code::PBOX => "PBOX",
            AddressType2Code::HOME => "HOME",
            AddressType2Code::BIZZ => "BIZZ",
            AddressType2Code::MLTO => "MLTO",
            AddressType2Code::DLVY => "DLVY",
            _ => "UNKNOWN"
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "ADDR" => Some(AddressType2Code::ADDR),
            "PBOX" => Some(AddressType2Code::PBOX),
            "HOME" => Some(AddressType2Code::HOME),
            "BIZZ" => Some(AddressType2Code::BIZZ),
            "MLTO" => Some(AddressType2Code::MLTO),
            "DLVY" => Some(AddressType2Code::DLVY),
            _ => None
        }
    }
}