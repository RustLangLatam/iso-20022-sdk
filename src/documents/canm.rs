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

// Re-export the iso 20022 canm module
pub use iso_20022_canm::*;

use super::Dmkr;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum Document {
    // canm
    canm_001_001_03(iso_20022_canm::canm_001_001_03::Document<Dmkr>),
    canm_002_001_03(iso_20022_canm::canm_002_001_03::Document<Dmkr>),
    canm_003_001_03(iso_20022_canm::canm_003_001_03::Document<Dmkr>),
    canm_004_001_03(iso_20022_canm::canm_004_001_03::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // canm
            "canm.001.001.03" => Document::canm_001_001_03(Default::default()),
            "canm.002.001.03" => Document::canm_002_001_03(Default::default()),
            "canm.003.001.03" => Document::canm_003_001_03(Default::default()),
            "canm.004.001.03" => Document::canm_004_001_03(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
