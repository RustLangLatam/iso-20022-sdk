#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct MandateRelatedData2Choice {
    #[serde(rename = "DrctDbtMndt", skip_serializing_if = "Option::is_none")]
    pub drct_dbt_mndt: Option<super::MandateRelatedInformation15>,
    #[serde(rename = "CdtTrfMndt", skip_serializing_if = "Option::is_none")]
    pub cdt_trf_mndt: Option<super::CreditTransferMandateData1>,
}