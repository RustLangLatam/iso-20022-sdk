use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct InstructionForCreditorAgent3 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub cd: Option<super::super::simple_type::ExternalCreditorAgentInstruction1Code>,
    #[serde(rename = "InstrInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub instr_inf: Option<super::super::simple_type::Max140Text>,
}