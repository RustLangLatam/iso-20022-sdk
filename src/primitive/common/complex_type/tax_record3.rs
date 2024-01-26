use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TaxRecord3 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub tp: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "Ctgy", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ctgy: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "CtgyDtls", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ctgy_dtls: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "DbtrSts", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dbtr_sts: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "CertId", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cert_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub frms_cd: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub prd: Option<super::TaxPeriod3>,
    #[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub tax_amt: Option<super::TaxAmount3>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub addtl_inf: Option<super::super::simple_type::Max140Text>,
}