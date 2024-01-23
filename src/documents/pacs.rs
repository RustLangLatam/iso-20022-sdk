// Copyright 2023 Emergent Financial, LLC - All Rights Reserved
//
//
// This software is licensed under the Emergent Financial Limited Public License Version 1.0
// (EF-LPLv1). You may use, copy, modify, and distribute this software under the terms and
// conditions of the EF-LPL. For more information, please refer to the full text of the license
// at https://github.com/emergentfinancial/ef-lpl.
//
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use super::{Dmkr, DEFAULT_XLMNS_PREFIX};
use crate::models;
use quick_xml::DeError;
use crate::primitive::Xmlns;

// Re-export the iso 20022 pacs module

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Display, EnumIter, EnumAsInner)]
pub enum Document {
    // pacs
    pacs_002_001_12(models::pacs::pacs_002_001_12::Document<Dmkr>),
    pacs_003_001_09(models::pacs::pacs_003_001_09::Document<Dmkr>),
    pacs_004_001_11(models::pacs::pacs_004_001_11::Document<Dmkr>),
    pacs_007_001_11(models::pacs::pacs_007_001_11::Document<Dmkr>),
    pacs_008_001_10(models::pacs::pacs_008_001_10::Document<Dmkr>),
    pacs_009_001_10(models::pacs::pacs_009_001_10::Document<Dmkr>),
    pacs_010_001_05(models::pacs::pacs_010_001_05::Document<Dmkr>),
    pacs_028_001_05(models::pacs::pacs_028_001_05::Document<Dmkr>),
    // pacs_029_001_01(iso_20022::pacs::pacs_029_001_01::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = DeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use crate::utils::XmlExt;
        let xmlns: Xmlns = quick_xml::de::from_str(s)?;
        let schema = xmlns.replace(DEFAULT_XLMNS_PREFIX, "");

        let doc = match schema.as_str() {
            "pacs.002.001.12" => Document::pacs_002_001_12(XmlExt::from_xml(s)?),
            "pacs.003.001.09" => Document::pacs_003_001_09(XmlExt::from_xml(s)?),
            "pacs.004.001.11" => Document::pacs_004_001_11(XmlExt::from_xml(s)?),
            "pacs.007.001.11" => Document::pacs_007_001_11(XmlExt::from_xml(s)?),
            "pacs.008.001.10" => Document::pacs_008_001_10(XmlExt::from_xml(s)?),
            "pacs.009.001.10" => Document::pacs_009_001_10(XmlExt::from_xml(s)?),
            "pacs.010.001.05" => Document::pacs_010_001_05(XmlExt::from_xml(s)?),
            "pacs.028.001.05" => Document::pacs_028_001_05(XmlExt::from_xml(s)?),
            // "pacs.029.001.01" => Document::pacs_029_001_01(Default::default()),
            _ => return Err(DeError::Custom(schema)),
        };

        Ok(doc)
    }
}
