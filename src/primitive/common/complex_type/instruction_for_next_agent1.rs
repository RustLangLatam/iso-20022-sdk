use ::validator::Validate;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct InstructionForNextAgent1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<super::super::simple_type::Instruction4Code>,
    #[serde(rename = "InstrInf", skip_serializing_if = "Option::is_none")]
    #[validate]
    pub instr_inf: Option<super::super::simple_type::Max140Text>,
}
