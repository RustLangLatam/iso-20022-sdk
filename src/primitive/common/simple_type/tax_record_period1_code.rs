#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum TaxRecordPeriod1Code {
    MM01,
    MM02,
    MM03,
    MM04,
    MM05,
    MM06,
    MM07,
    MM08,
    MM09,
    MM10,
    MM11,
    MM12,
    QTR1,
    QTR2,
    QTR3,
    QTR4,
    HLF1,
    HLF2,
    #[default]
    UNKNOWN,
}

impl TaxRecordPeriod1Code {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TaxRecordPeriod1Code::MM01 => "MM01",
            TaxRecordPeriod1Code::MM02 => "MM02",
            TaxRecordPeriod1Code::MM03 => "MM03",
            TaxRecordPeriod1Code::MM04 => "MM04",
            TaxRecordPeriod1Code::MM05 => "MM05",
            TaxRecordPeriod1Code::MM06 => "MM06",
            TaxRecordPeriod1Code::MM07 => "MM07",
            TaxRecordPeriod1Code::MM08 => "MM08",
            TaxRecordPeriod1Code::MM09 => "MM09",
            TaxRecordPeriod1Code::MM10 => "MM10",
            TaxRecordPeriod1Code::MM11 => "MM11",
            TaxRecordPeriod1Code::MM12 => "MM12",
            TaxRecordPeriod1Code::QTR1 => "QTR1",
            TaxRecordPeriod1Code::QTR2 => "QTR2",
            TaxRecordPeriod1Code::QTR3 => "QTR3",
            TaxRecordPeriod1Code::QTR4 => "QTR4",
            TaxRecordPeriod1Code::HLF1 => "HLF1",
            TaxRecordPeriod1Code::HLF2 => "HLF2",
            _ => "UNKNOWN",
        }
    }

    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "MM01" => Some(TaxRecordPeriod1Code::MM01),
            "MM02" => Some(TaxRecordPeriod1Code::MM02),
            "MM03" => Some(TaxRecordPeriod1Code::MM03),
            "MM04" => Some(TaxRecordPeriod1Code::MM04),
            "MM05" => Some(TaxRecordPeriod1Code::MM05),
            "MM06" => Some(TaxRecordPeriod1Code::MM06),
            "MM07" => Some(TaxRecordPeriod1Code::MM07),
            "MM08" => Some(TaxRecordPeriod1Code::MM08),
            "MM09" => Some(TaxRecordPeriod1Code::MM09),
            "MM10" => Some(TaxRecordPeriod1Code::MM10),
            "MM11" => Some(TaxRecordPeriod1Code::MM11),
            "MM12" => Some(TaxRecordPeriod1Code::MM12),
            "QTR1" => Some(TaxRecordPeriod1Code::QTR1),
            "QTR2" => Some(TaxRecordPeriod1Code::QTR2),
            "QTR3" => Some(TaxRecordPeriod1Code::QTR3),
            "QTR4" => Some(TaxRecordPeriod1Code::QTR4),
            "HLF1" => Some(TaxRecordPeriod1Code::HLF1),
            "HLF2" => Some(TaxRecordPeriod1Code::HLF2),
            _ => None,
        }
    }
}
