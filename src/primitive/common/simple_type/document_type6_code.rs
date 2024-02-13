#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum DocumentType6Code {
    MSIN,
    CNFA,
    DNFA,
    CINV,
    CREN,
    DEBN,
    HIRI,
    SBIN,
    CMCN,
    SOAC,
    DISP,
    BOLD,
    VCHR,
    AROI,
    TSUT,
    PUOR,
    #[default]
    UNKNOWN,
}

impl DocumentType6Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DocumentType6Code::MSIN => "MSIN",
            DocumentType6Code::CNFA => "CNFA",
            DocumentType6Code::DNFA => "DNFA",
            DocumentType6Code::CINV => "CINV",
            DocumentType6Code::CREN => "CREN",
            DocumentType6Code::DEBN => "DEBN",
            DocumentType6Code::HIRI => "HIRI",
            DocumentType6Code::SBIN => "SBIN",
            DocumentType6Code::CMCN => "CMCN",
            DocumentType6Code::SOAC => "SOAC",
            DocumentType6Code::DISP => "DISP",
            DocumentType6Code::BOLD => "BOLD",
            DocumentType6Code::VCHR => "VCHR",
            DocumentType6Code::AROI => "AROI",
            DocumentType6Code::TSUT => "TSUT",
            DocumentType6Code::PUOR => "PUOR",
            _ => "UNKNOWN",
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "MSIN" => Some(DocumentType6Code::MSIN),
            "CNFA" => Some(DocumentType6Code::CNFA),
            "DNFA" => Some(DocumentType6Code::DNFA),
            "CINV" => Some(DocumentType6Code::CINV),
            "CREN" => Some(DocumentType6Code::CREN),
            "DEBN" => Some(DocumentType6Code::DEBN),
            "HIRI" => Some(DocumentType6Code::HIRI),
            "SBIN" => Some(DocumentType6Code::SBIN),
            "CMCN" => Some(DocumentType6Code::CMCN),
            "SOAC" => Some(DocumentType6Code::SOAC),
            "DISP" => Some(DocumentType6Code::DISP),
            "BOLD" => Some(DocumentType6Code::BOLD),
            "VCHR" => Some(DocumentType6Code::VCHR),
            "AROI" => Some(DocumentType6Code::AROI),
            "TSUT" => Some(DocumentType6Code::TSUT),
            "PUOR" => Some(DocumentType6Code::PUOR),
            _ => None,
        }
    }
}
