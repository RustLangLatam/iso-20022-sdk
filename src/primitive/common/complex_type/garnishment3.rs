use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct Garnishment3 {
    #[serde(rename = "Tp")]
    #[validate]
    pub tp: super::GarnishmentType1,
    #[serde(rename = "Grnshee", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub grnshee: Option<super::PartyIdentification135>,
    #[serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub grnshmt_admstr: Option<super::PartyIdentification135>,
    #[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub ref_nb: Option<super::super::simple_type::Max140Text>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub rmtd_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "FmlyMdclInsrncInd", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub fmly_mdcl_insrnc_ind: Option<super::super::simple_type::TrueFalseIndicator>,
    #[serde(rename = "MplyeeTermntnInd", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub mplyee_termntn_ind: Option<super::super::simple_type::TrueFalseIndicator>,
}