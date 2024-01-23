#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct StructuredRemittanceInformation17 {
    #[serde(rename = "RfrdDocInf")]
    #[validate(length(min = 0,))]
    #[serde(default)]
    pub rfrd_doc_inf: Vec<super::ReferredDocumentInformation7>,
    #[serde(rename = "RfrdDocAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfrd_doc_amt: Option<super::RemittanceAmount2>,
    #[serde(rename = "CdtrRefInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_ref_inf: Option<super::CreditorReferenceInformation2>,
    #[serde(rename = "Invcr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invcr: Option<super::PartyIdentification135>,
    #[serde(rename = "Invcee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invcee: Option<super::PartyIdentification135>,
    #[serde(rename = "TaxRmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rmt: Option<super::TaxData1>,
    #[serde(rename = "GrnshmtRmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grnshmt_rmt: Option<super::Garnishment3>,
    #[serde(rename = "AddtlRmtInf")]
    #[validate(length(min = 0, max = 3))]
    #[serde(default)]
    pub addtl_rmt_inf: Vec<super::super::simple_type::Max140Text>,
}