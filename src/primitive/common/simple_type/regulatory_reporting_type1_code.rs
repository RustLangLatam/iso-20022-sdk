#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RegulatoryReportingType1Code {
    CRED,
    DEBT,
    BOTH,
    #[default]
    UNKNOWN
}

impl RegulatoryReportingType1Code {

    pub fn as_str_name(&self) -> &'static str {
        match self {
            RegulatoryReportingType1Code::CRED => "CRED",
            RegulatoryReportingType1Code::DEBT => "DEBT",
            RegulatoryReportingType1Code::BOTH => "BOTH",
            _ => "UNKNOWN"
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "CRED" => Some(RegulatoryReportingType1Code::CRED),
            "DEBT" => Some(RegulatoryReportingType1Code::DEBT),
            "BOTH" => Some(RegulatoryReportingType1Code::BOTH),
            _ => None
        }
    }
}