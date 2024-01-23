use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct Garnishment3 {
    #[serde(rename = "Tp")]
    #[validate]
    pub tp: super::GarnishmentType1,
    #[serde(rename = "Grnshee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grnshee: Option<super::PartyIdentification135>,
    #[serde(rename = "GrnshmtAdmstr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grnshmt_admstr: Option<super::PartyIdentification135>,
    #[serde(rename = "RefNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_nb: Option<super::super::simple_type::Max140Text>,
    #[serde(rename = "Dt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt: Option<super::super::simple_type::ISODate>,
    #[serde(rename = "RmtdAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmtd_amt: Option<super::ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "FmlyMdclInsrncInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fmly_mdcl_insrnc_ind: Option<super::super::simple_type::TrueFalseIndicator>,
    #[serde(rename = "MplyeeTermntnInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mplyee_termntn_ind: Option<super::super::simple_type::TrueFalseIndicator>,
}