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

// Re-export the iso 20022 cafc module
pub use iso_20022_cafc::*;

use super::Dmkr;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum Document {
    // cafc
    cafc_001_001_02(iso_20022_cafc::cafc_001_001_02::Document<Dmkr>),
    cafc_002_001_02(iso_20022_cafc::cafc_002_001_02::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // cafc
            "cafc.001.001.02" => Document::cafc_001_001_02(Default::default()),
            "cafc.002.001.02" => Document::cafc_002_001_02(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
