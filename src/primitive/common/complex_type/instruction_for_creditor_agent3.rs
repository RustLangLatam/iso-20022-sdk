#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct InstructionForCreditorAgent3 {
    #[serde(rename = "Cd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cd: Option<super::super::simple_type::ExternalCreditorAgentInstruction1Code>,
    #[serde(rename = "InstrInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_inf: Option<super::super::simple_type::Max140Text>,
}