//! # Payment & Clearing Settlement (PACS) Module
//!
//! The `pacs` module provides functionality related to payment and clearing settlement
//! in accordance with the ISO 20022 standard.
//!
//! "Payments Clearing and Settlement - Maintenance 2020 - 2021"
//! refers to a specific set of updates and revisions made to the ISO 20022 messaging standard in the domain of Payments,
//! Clearing, and Settlement during the maintenance period from 2020 to 2021.
//!

pub mod pacs_002_001_12;
pub mod pacs_004_001_11;
pub mod pacs_009_001_10;
pub mod pacs_003_001_09;
pub mod pacs_007_001_11;
pub mod pacs_010_001_05;
pub mod pacs_008_001_10;
pub mod pacs_028_001_05;

#[cfg(test)]
mod tests;
