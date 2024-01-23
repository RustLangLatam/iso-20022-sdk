#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct InstructionForNextAgent1 {
    #[serde(rename = "Cd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cd: Option<super::super::simple_type::Instruction4Code>,
    #[serde(rename = "InstrInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_inf: Option<super::super::simple_type::Max140Text>,
}