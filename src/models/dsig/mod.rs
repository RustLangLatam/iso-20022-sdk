pub use self::dsig::*;
pub use self::ecdsa::*;
pub use self::xpath::*;

/// XML Signature Syntax and Processing Version 2.0
pub mod dsig;

/// Using the Elliptic Curve Signature Algorithm (ECDSA)
/// for XML Digital Signatures. IETF RFC 4050.
pub mod ecdsa;

/// XML-Signature XPath Filter 2.0
/// See https://www.w3.org/TR/xmldsig-filter2/
pub mod xpath;