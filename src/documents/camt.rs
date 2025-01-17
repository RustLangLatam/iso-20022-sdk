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

// Re-export the iso 20022 camt module
pub use iso_20022_camt::*;

use super::Dmkr;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum Document {
    // camt
    camt_003_001_07(iso_20022_camt::camt_003_001_07::Document<Dmkr>),
    camt_004_001_09(iso_20022_camt::camt_004_001_09::Document<Dmkr>),
    camt_005_001_09(iso_20022_camt::camt_005_001_09::Document<Dmkr>),
    camt_006_001_09(iso_20022_camt::camt_006_001_09::Document<Dmkr>),
    camt_007_001_08(iso_20022_camt::camt_007_001_08::Document<Dmkr>),
    camt_008_001_09(iso_20022_camt::camt_008_001_09::Document<Dmkr>),
    camt_009_001_07(iso_20022_camt::camt_009_001_07::Document<Dmkr>),
    camt_010_001_08(iso_20022_camt::camt_010_001_08::Document<Dmkr>),
    camt_011_001_07(iso_20022_camt::camt_011_001_07::Document<Dmkr>),
    camt_012_001_07(iso_20022_camt::camt_012_001_07::Document<Dmkr>),
    camt_013_001_04(iso_20022_camt::camt_013_001_04::Document<Dmkr>),
    camt_014_001_05(iso_20022_camt::camt_014_001_05::Document<Dmkr>),
    camt_015_001_04(iso_20022_camt::camt_015_001_04::Document<Dmkr>),
    camt_016_001_04(iso_20022_camt::camt_016_001_04::Document<Dmkr>),
    camt_017_001_05(iso_20022_camt::camt_017_001_05::Document<Dmkr>),
    camt_018_001_05(iso_20022_camt::camt_018_001_05::Document<Dmkr>),
    camt_019_001_07(iso_20022_camt::camt_019_001_07::Document<Dmkr>),
    camt_020_001_04(iso_20022_camt::camt_020_001_04::Document<Dmkr>),
    camt_021_001_06(iso_20022_camt::camt_021_001_06::Document<Dmkr>),
    camt_023_001_07(iso_20022_camt::camt_023_001_07::Document<Dmkr>),
    camt_024_001_07(iso_20022_camt::camt_024_001_07::Document<Dmkr>),
    camt_025_001_05(iso_20022_camt::camt_025_001_05::Document<Dmkr>),
    camt_026_001_09(iso_20022_camt::camt_026_001_09::Document<Dmkr>),
    camt_027_001_09(iso_20022_camt::camt_027_001_09::Document<Dmkr>),
    camt_028_001_11(iso_20022_camt::camt_028_001_11::Document<Dmkr>),
    camt_029_001_11(iso_20022_camt::camt_029_001_11::Document<Dmkr>),
    camt_030_001_05(iso_20022_camt::camt_030_001_05::Document<Dmkr>),
    camt_031_001_06(iso_20022_camt::camt_031_001_06::Document<Dmkr>),
    camt_032_001_04(iso_20022_camt::camt_032_001_04::Document<Dmkr>),
    camt_033_001_06(iso_20022_camt::camt_033_001_06::Document<Dmkr>),
    camt_034_001_06(iso_20022_camt::camt_034_001_06::Document<Dmkr, Dmkr>),
    camt_035_001_05(iso_20022_camt::camt_035_001_05::Document<Dmkr, Dmkr>),
    camt_036_001_05(iso_20022_camt::camt_036_001_05::Document<Dmkr>),
    camt_037_001_09(iso_20022_camt::camt_037_001_09::Document<Dmkr>),
    camt_038_001_04(iso_20022_camt::camt_038_001_04::Document<Dmkr>),
    camt_039_001_05(iso_20022_camt::camt_039_001_05::Document<Dmkr>),
    camt_040_001_04(iso_20022_camt::camt_040_001_04::Document),
    camt_041_001_04(iso_20022_camt::camt_041_001_04::Document),
    camt_042_001_04(iso_20022_camt::camt_042_001_04::Document),
    camt_043_001_04(iso_20022_camt::camt_043_001_04::Document),
    camt_044_001_03(iso_20022_camt::camt_044_001_03::Document),
    camt_045_001_03(iso_20022_camt::camt_045_001_03::Document),
    camt_046_001_06(iso_20022_camt::camt_046_001_06::Document<Dmkr>),
    camt_047_001_07(iso_20022_camt::camt_047_001_07::Document<Dmkr>),
    camt_048_001_06(iso_20022_camt::camt_048_001_06::Document<Dmkr>),
    camt_049_001_06(iso_20022_camt::camt_049_001_06::Document<Dmkr>),
    camt_050_001_06(iso_20022_camt::camt_050_001_06::Document<Dmkr>),
    camt_051_001_05(iso_20022_camt::camt_051_001_05::Document<Dmkr>),
    camt_052_001_10(iso_20022_camt::camt_052_001_10::Document<Dmkr, Dmkr>),
    camt_053_001_10(iso_20022_camt::camt_053_001_10::Document<Dmkr, Dmkr>),
    camt_054_001_10(iso_20022_camt::camt_054_001_10::Document<Dmkr, Dmkr>),
    camt_055_001_11(iso_20022_camt::camt_055_001_11::Document<Dmkr, Dmkr>),
    camt_056_001_10(iso_20022_camt::camt_056_001_10::Document<Dmkr, Dmkr>),
    camt_057_001_07(iso_20022_camt::camt_057_001_07::Document<Dmkr>),
    camt_058_001_08(iso_20022_camt::camt_058_001_08::Document<Dmkr>),
    camt_059_001_07(iso_20022_camt::camt_059_001_07::Document<Dmkr>),
    camt_060_001_06(iso_20022_camt::camt_060_001_06::Document<Dmkr>),
    camt_061_001_02(iso_20022_camt::camt_061_001_02::Document<Dmkr>),
    camt_062_001_03(iso_20022_camt::camt_062_001_03::Document<Dmkr>),
    camt_063_001_02(iso_20022_camt::camt_063_001_02::Document<Dmkr>),
    camt_066_001_01(iso_20022_camt::camt_066_001_01::Document<Dmkr>),
    camt_067_001_01(iso_20022_camt::camt_067_001_01::Document<Dmkr>),
    camt_068_001_01(iso_20022_camt::camt_068_001_01::Document<Dmkr>),
    camt_069_001_04(iso_20022_camt::camt_069_001_04::Document<Dmkr>),
    camt_070_001_05(iso_20022_camt::camt_070_001_05::Document<Dmkr>),
    camt_071_001_04(iso_20022_camt::camt_071_001_04::Document<Dmkr>),
    camt_072_001_01(iso_20022_camt::camt_072_001_01::Document<Dmkr>),
    camt_073_001_01(iso_20022_camt::camt_073_001_01::Document<Dmkr>),
    camt_074_001_01(iso_20022_camt::camt_074_001_01::Document<Dmkr>),
    camt_075_001_01(iso_20022_camt::camt_075_001_01::Document<Dmkr>),
    camt_078_001_01(iso_20022_camt::camt_078_001_01::Document<Dmkr>),
    camt_079_001_01(iso_20022_camt::camt_079_001_01::Document<Dmkr>),
    camt_080_001_01(iso_20022_camt::camt_080_001_01::Document<Dmkr>),
    camt_081_001_01(iso_20022_camt::camt_081_001_01::Document<Dmkr>),
    camt_082_001_01(iso_20022_camt::camt_082_001_01::Document<Dmkr>),
    camt_083_001_01(iso_20022_camt::camt_083_001_01::Document<Dmkr>),
    camt_084_001_01(iso_20022_camt::camt_084_001_01::Document<Dmkr>),
    camt_085_001_01(iso_20022_camt::camt_085_001_01::Document<Dmkr>),
    camt_086_001_04(iso_20022_camt::camt_086_001_04::Document),
    camt_087_001_08(iso_20022_camt::camt_087_001_08::Document<Dmkr>),
    camt_088_001_01(iso_20022_camt::camt_088_001_01::Document<Dmkr>),
    camt_101_001_01(iso_20022_camt::camt_101_001_01::Document<Dmkr>),
    camt_102_001_02(iso_20022_camt::camt_102_001_02::Document<Dmkr>),
    camt_103_001_02(iso_20022_camt::camt_103_001_02::Document<Dmkr>),
    camt_104_001_01(iso_20022_camt::camt_104_001_01::Document<Dmkr>),
    camt_105_001_01(iso_20022_camt::camt_105_001_01::Document<Dmkr>),
    camt_106_001_01(iso_20022_camt::camt_106_001_01::Document<Dmkr>),
    camt_107_001_01(iso_20022_camt::camt_107_001_01::Document<Dmkr>),
    camt_108_001_01(iso_20022_camt::camt_108_001_01::Document<Dmkr>),
    camt_109_001_01(iso_20022_camt::camt_109_001_01::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // camt
            "camt.003.001.07" => Document::camt_003_001_07(Default::default()),
            "camt.004.001.09" => Document::camt_004_001_09(Default::default()),
            "camt.005.001.09" => Document::camt_005_001_09(Default::default()),
            "camt.006.001.09" => Document::camt_006_001_09(Default::default()),
            "camt.007.001.08" => Document::camt_007_001_08(Default::default()),
            "camt.008.001.09" => Document::camt_008_001_09(Default::default()),
            "camt.009.001.07" => Document::camt_009_001_07(Default::default()),
            "camt.010.001.08" => Document::camt_010_001_08(Default::default()),
            "camt.011.001.07" => Document::camt_011_001_07(Default::default()),
            "camt.012.001.07" => Document::camt_012_001_07(Default::default()),
            "camt.013.001.04" => Document::camt_013_001_04(Default::default()),
            "camt.014.001.05" => Document::camt_014_001_05(Default::default()),
            "camt.015.001.04" => Document::camt_015_001_04(Default::default()),
            "camt.016.001.04" => Document::camt_016_001_04(Default::default()),
            "camt.017.001.05" => Document::camt_017_001_05(Default::default()),
            "camt.018.001.05" => Document::camt_018_001_05(Default::default()),
            "camt.019.001.07" => Document::camt_019_001_07(Default::default()),
            "camt.020.001.04" => Document::camt_020_001_04(Default::default()),
            "camt.021.001.06" => Document::camt_021_001_06(Default::default()),
            "camt.023.001.07" => Document::camt_023_001_07(Default::default()),
            "camt.024.001.07" => Document::camt_024_001_07(Default::default()),
            "camt.025.001.05" => Document::camt_025_001_05(Default::default()),
            "camt.026.001.09" => Document::camt_026_001_09(Default::default()),
            "camt.027.001.09" => Document::camt_027_001_09(Default::default()),
            "camt.028.001.11" => Document::camt_028_001_11(Default::default()),
            "camt.029.001.11" => Document::camt_029_001_11(Default::default()),
            "camt.030.001.05" => Document::camt_030_001_05(Default::default()),
            "camt.031.001.06" => Document::camt_031_001_06(Default::default()),
            "camt.032.001.04" => Document::camt_032_001_04(Default::default()),
            "camt.033.001.06" => Document::camt_033_001_06(Default::default()),
            "camt.034.001.06" => Document::camt_034_001_06(Default::default()),
            "camt.035.001.05" => Document::camt_035_001_05(Default::default()),
            "camt.036.001.05" => Document::camt_036_001_05(Default::default()),
            "camt.037.001.09" => Document::camt_037_001_09(Default::default()),
            "camt.038.001.04" => Document::camt_038_001_04(Default::default()),
            "camt.039.001.05" => Document::camt_039_001_05(Default::default()),
            "camt.040.001.04" => Document::camt_040_001_04(Default::default()),
            "camt.041.001.04" => Document::camt_041_001_04(Default::default()),
            "camt.042.001.04" => Document::camt_042_001_04(Default::default()),
            "camt.043.001.04" => Document::camt_043_001_04(Default::default()),
            "camt.044.001.03" => Document::camt_044_001_03(Default::default()),
            "camt.045.001.03" => Document::camt_045_001_03(Default::default()),
            "camt.046.001.06" => Document::camt_046_001_06(Default::default()),
            "camt.047.001.07" => Document::camt_047_001_07(Default::default()),
            "camt.048.001.06" => Document::camt_048_001_06(Default::default()),
            "camt.049.001.06" => Document::camt_049_001_06(Default::default()),
            "camt.050.001.06" => Document::camt_050_001_06(Default::default()),
            "camt.051.001.05" => Document::camt_051_001_05(Default::default()),
            "camt.052.001.10" => Document::camt_052_001_10(Default::default()),
            "camt.053.001.10" => Document::camt_053_001_10(Default::default()),
            "camt.054.001.10" => Document::camt_054_001_10(Default::default()),
            "camt.055.001.11" => Document::camt_055_001_11(Default::default()),
            "camt.056.001.10" => Document::camt_056_001_10(Default::default()),
            "camt.057.001.07" => Document::camt_057_001_07(Default::default()),
            "camt.058.001.08" => Document::camt_058_001_08(Default::default()),
            "camt.059.001.07" => Document::camt_059_001_07(Default::default()),
            "camt.060.001.06" => Document::camt_060_001_06(Default::default()),
            "camt.061.001.02" => Document::camt_061_001_02(Default::default()),
            "camt.062.001.03" => Document::camt_062_001_03(Default::default()),
            "camt.063.001.02" => Document::camt_063_001_02(Default::default()),
            "camt.066.001.01" => Document::camt_066_001_01(Default::default()),
            "camt.067.001.01" => Document::camt_067_001_01(Default::default()),
            "camt.068.001.01" => Document::camt_068_001_01(Default::default()),
            "camt.069.001.04" => Document::camt_069_001_04(Default::default()),
            "camt.070.001.05" => Document::camt_070_001_05(Default::default()),
            "camt.071.001.04" => Document::camt_071_001_04(Default::default()),
            "camt.072.001.01" => Document::camt_072_001_01(Default::default()),
            "camt.073.001.01" => Document::camt_073_001_01(Default::default()),
            "camt.074.001.01" => Document::camt_074_001_01(Default::default()),
            "camt.075.001.01" => Document::camt_075_001_01(Default::default()),
            "camt.078.001.01" => Document::camt_078_001_01(Default::default()),
            "camt.079.001.01" => Document::camt_079_001_01(Default::default()),
            "camt.080.001.01" => Document::camt_080_001_01(Default::default()),
            "camt.081.001.01" => Document::camt_081_001_01(Default::default()),
            "camt.082.001.01" => Document::camt_082_001_01(Default::default()),
            "camt.083.001.01" => Document::camt_083_001_01(Default::default()),
            "camt.084.001.01" => Document::camt_084_001_01(Default::default()),
            "camt.085.001.01" => Document::camt_085_001_01(Default::default()),
            "camt.086.001.04" => Document::camt_086_001_04(Default::default()),
            "camt.087.001.08" => Document::camt_087_001_08(Default::default()),
            "camt.088.001.01" => Document::camt_088_001_01(Default::default()),
            "camt.101.001.01" => Document::camt_101_001_01(Default::default()),
            "camt.102.001.02" => Document::camt_102_001_02(Default::default()),
            "camt.103.001.02" => Document::camt_103_001_02(Default::default()),
            "camt.104.001.01" => Document::camt_104_001_01(Default::default()),
            "camt.105.001.01" => Document::camt_105_001_01(Default::default()),
            "camt.106.001.01" => Document::camt_106_001_01(Default::default()),
            "camt.107.001.01" => Document::camt_107_001_01(Default::default()),
            "camt.108.001.01" => Document::camt_108_001_01(Default::default()),
            "camt.109.001.01" => Document::camt_109_001_01(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
