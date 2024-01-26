use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct StructuredRemittanceInformation17 {
    #[serde(default, rename = "RfrdDocInf", skip_serializing_if = "<[_]>::is_empty")]
    #[validate]
    pub rfrd_doc_inf: Vec<super::ReferredDocumentInformation7>,
    #[serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rfrd_doc_amt: Option<super::RemittanceAmount2>,
    #[serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cdtr_ref_inf: Option<super::CreditorReferenceInformation2>,
    #[serde(rename = "Invcr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub invcr: Option<super::PartyIdentification135>,
    #[serde(rename = "Invcee", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub invcee: Option<super::PartyIdentification135>,
    #[serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub tax_rmt: Option<super::TaxData1>,
    #[serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub grnshmt_rmt: Option<super::Garnishment3>,
    #[serde(default, rename = "AddtlRmtInf", skip_serializing_if = "<[_]>::is_empty")]
    #[validate(length(min = 0, max = 3))]
    pub addtl_rmt_inf: Vec<super::super::simple_type::Max140Text>,
}