#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PaymentMethod4Code {
    CHK,
    TRF,
    DD,
    TRA,
    #[default]
    UNKNOWN
}

impl PaymentMethod4Code {

    pub fn as_str_name(&self) -> &'static str {
        match self {
            PaymentMethod4Code::CHK => "CHK",
            PaymentMethod4Code::TRF => "TRF",
            PaymentMethod4Code::DD => "DD",
            PaymentMethod4Code::TRA => "TRA",
            _ => "UNKNOWN"
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "CHK" => Some(PaymentMethod4Code::CHK),
            "TRF" => Some(PaymentMethod4Code::TRF),
            "DD" => Some(PaymentMethod4Code::DD),
            "TRA" => Some(PaymentMethod4Code::TRA),
            _ => None
        }
    }
}