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

// Re-export the iso 20022 fxtr module
pub use iso_20022_fxtr::*;

use super::Dmkr;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum Document {
    // fxtr
    fxtr_008_001_06(iso_20022_fxtr::fxtr_008_001_06::Document<Dmkr>),
    fxtr_013_001_03(iso_20022_fxtr::fxtr_013_001_03::Document<Dmkr>),
    fxtr_014_001_04(iso_20022_fxtr::fxtr_014_001_04::Document<Dmkr>),
    fxtr_015_001_04(iso_20022_fxtr::fxtr_015_001_04::Document<Dmkr>),
    fxtr_016_001_04(iso_20022_fxtr::fxtr_016_001_04::Document<Dmkr>),
    fxtr_017_001_04(iso_20022_fxtr::fxtr_017_001_04::Document<Dmkr>),
    fxtr_030_001_04(iso_20022_fxtr::fxtr_030_001_04::Document<Dmkr>),
    fxtr_031_001_01(iso_20022_fxtr::fxtr_031_001_01::Document<Dmkr>),
    fxtr_032_001_01(iso_20022_fxtr::fxtr_032_001_01::Document<Dmkr>),
    fxtr_033_001_01(iso_20022_fxtr::fxtr_033_001_01::Document<Dmkr>),
    fxtr_034_001_01(iso_20022_fxtr::fxtr_034_001_01::Document<Dmkr>),
    fxtr_035_001_01(iso_20022_fxtr::fxtr_035_001_01::Document<Dmkr>),
    fxtr_036_001_01(iso_20022_fxtr::fxtr_036_001_01::Document<Dmkr>),
    fxtr_037_001_01(iso_20022_fxtr::fxtr_037_001_01::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // fxtr
            "fxtr.008.001.06" => Document::fxtr_008_001_06(Default::default()),
            "fxtr.013.001.03" => Document::fxtr_013_001_03(Default::default()),
            "fxtr.014.001.04" => Document::fxtr_014_001_04(Default::default()),
            "fxtr.015.001.04" => Document::fxtr_015_001_04(Default::default()),
            "fxtr.016.001.04" => Document::fxtr_016_001_04(Default::default()),
            "fxtr.017.001.04" => Document::fxtr_017_001_04(Default::default()),
            "fxtr.030.001.04" => Document::fxtr_030_001_04(Default::default()),
            "fxtr.031.001.01" => Document::fxtr_031_001_01(Default::default()),
            "fxtr.032.001.01" => Document::fxtr_032_001_01(Default::default()),
            "fxtr.033.001.01" => Document::fxtr_033_001_01(Default::default()),
            "fxtr.034.001.01" => Document::fxtr_034_001_01(Default::default()),
            "fxtr.035.001.01" => Document::fxtr_035_001_01(Default::default()),
            "fxtr.036.001.01" => Document::fxtr_036_001_01(Default::default()),
            "fxtr.037.001.01" => Document::fxtr_037_001_01(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
