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

// Re-export the iso 20022 cafr module
pub use iso_20022_cafr::*;

use super::Dmkr;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum Document {
    // cafr
    cafr_001_001_02(iso_20022_cafr::cafr_001_001_02::Document<Dmkr>),
    cafr_002_001_02(iso_20022_cafr::cafr_002_001_02::Document<Dmkr>),
    cafr_003_001_02(iso_20022_cafr::cafr_003_001_02::Document<Dmkr>),
    cafr_004_001_02(iso_20022_cafr::cafr_004_001_02::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // cafr
            "cafr.001.001.02" => Document::cafr_001_001_02(Default::default()),
            "cafr.002.001.02" => Document::cafr_002_001_02(Default::default()),
            "cafr.003.001.02" => Document::cafr_003_001_02(Default::default()),
            "cafr.004.001.02" => Document::cafr_004_001_02(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
