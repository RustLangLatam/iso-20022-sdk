#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct TaxRecord3 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "Ctgy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctgy: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "CtgyDtls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctgy_dtls: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "DbtrSts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_sts: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "CertId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_id: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "FrmsCd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frms_cd: Option<super::super::simple_type::Max35Text>,
    #[serde(rename = "Prd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prd: Option<super::TaxPeriod3>,
    #[serde(rename = "TaxAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amt: Option<super::TaxAmount3>,
    #[serde(rename = "AddtlInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<super::super::simple_type::Max140Text>,
}