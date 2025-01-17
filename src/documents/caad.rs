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

// Re-export the iso 20022 caad module
pub use iso_20022_caad::*;

use super::Dmkr;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum Document {
    // caad
    caad_001_001_02(caad_001_001_02::Document<Dmkr>),
    caad_002_001_02(caad_002_001_02::Document<Dmkr>),
    caad_003_001_02(caad_003_001_02::Document<Dmkr>),
    caad_004_001_02(iso_20022_caad::caad_004_001_02::Document<Dmkr>),
    caad_005_001_03(iso_20022_caad::caad_005_001_03::Document<Dmkr>),
    caad_006_001_03(iso_20022_caad::caad_006_001_03::Document<Dmkr>),
    caad_007_001_03(iso_20022_caad::caad_007_001_03::Document<Dmkr>),
    caad_008_001_01(caad_008_001_01::Document<Dmkr>),
    caad_009_001_01(iso_20022_caad::caad_009_001_01::Document<Dmkr>),
    caad_010_001_01(iso_20022_caad::caad_010_001_01::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // caad
            "caad.001.001.02" => Document::caad_001_001_02(Default::default()),
            "caad.002.001.02" => Document::caad_002_001_02(Default::default()),
            "caad.003.001.02" => Document::caad_003_001_02(Default::default()),
            "caad.004.001.02" => Document::caad_004_001_02(Default::default()),
            "caad.005.001.03" => Document::caad_005_001_03(Default::default()),
            "caad.006.001.03" => Document::caad_006_001_03(Default::default()),
            "caad.007.001.03" => Document::caad_007_001_03(Default::default()),
            "caad.008.001.01" => Document::caad_008_001_01(Default::default()),
            "caad.009.001.01" => Document::caad_009_001_01(Default::default()),
            "caad.010.001.01" => Document::caad_010_001_01(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
