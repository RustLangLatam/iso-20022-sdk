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

// Re-export the iso 20022 catm module
pub use iso_20022_catm::*;

use super::Dmkr;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum Document {
    // catm
    catm_001_001_11(iso_20022_catm::catm_001_001_11::Document<Dmkr>),
    catm_002_001_10(iso_20022_catm::catm_002_001_10::Document<Dmkr>),
    catm_003_001_11(iso_20022_catm::catm_003_001_11::Document),
    catm_004_001_05(iso_20022_catm::catm_004_001_05::Document),
    catm_005_001_08(iso_20022_catm::catm_005_001_08::Document<Dmkr>),
    catm_006_001_06(iso_20022_catm::catm_006_001_06::Document),
    catm_007_001_05(iso_20022_catm::catm_007_001_05::Document),
    catm_008_001_05(iso_20022_catm::catm_008_001_05::Document),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // catm
            "catm.001.001.11" => Document::catm_001_001_11(Default::default()),
            "catm.002.001.10" => Document::catm_002_001_10(Default::default()),
            "catm.003.001.11" => Document::catm_003_001_11(Default::default()),
            "catm.004.001.05" => Document::catm_004_001_05(Default::default()),
            "catm.005.001.08" => Document::catm_005_001_08(Default::default()),
            "catm.006.001.06" => Document::catm_006_001_06(Default::default()),
            "catm.007.001.05" => Document::catm_007_001_05(Default::default()),
            "catm.008.001.05" => Document::catm_008_001_05(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
