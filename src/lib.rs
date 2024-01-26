// The `quote!` macro requires deep recursion.
#![recursion_limit = "4096"]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate enum_as_inner;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate validator;
// #[macro_use]
// extern crate iso_20022_macros;

#[cfg(feature = "crypto")]
pub mod crypto;

pub mod external_codes;

pub mod models;

pub mod primitive;

pub mod documents;

pub(crate) mod utils;

pub mod prelude {
    #[cfg(feature = "crypto")]
    pub use super::crypto::*;
    pub use super::documents::*;
    #[cfg(feature = "dsig")]
    pub use super::models::dsig;
    //
    // #[cfg(feature = "nvlp")]
    // pub use super::models::nvlp;
    //
    // #[cfg(feature = "msg")]
    // pub use super::message::*;
    //
    #[cfg(feature = "head")]
    pub use super::models::head;
    #[cfg(feature = "pacs")]
    pub use super::models::pacs;
}
