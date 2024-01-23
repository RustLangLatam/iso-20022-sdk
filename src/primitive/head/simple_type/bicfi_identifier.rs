// The BICFI_IDENTIFIER_REGEX is a regular expression defined using the regex crate in Rust. Let's break down the components of this regular expression:
// [A-Z]{6,6}: This part specifies that there should be exactly 6 uppercase letters from 'A' to 'Z'.
// [A-Z2-9]: This part allows for a single uppercase letter from 'A' to 'Z' or a digit from '2' to '9'.
// [A-NP-Z0-9]: This part allows for a single uppercase letter from 'A' to 'N' or 'P' to 'Z' or a digit from '0' to '9'.
// ([A-Z0-9]{3,3}){0,1}: This part is an optional group that allows for a sequence of exactly 3 uppercase letters or digits, repeated 0 or 1 times.
// So, the overall regular expression is defining a pattern for a BIC (Bank Identifier Code) or SWIFT code. BIC codes have a specific format,
// and this regular expression is designed to match strings that conform to that format, ensuring a certain structure and length for different sections of the code.

::lazy_static::lazy_static! {
    pub(super) static ref BICFI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct BICFIIdentifier {
    #[validate(regex = "BICFI_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}