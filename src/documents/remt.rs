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

// Re-export the iso 20022 remt module
pub use iso_20022_remt::*;

use super::Dmkr;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum Document {
    // remt
    #[serde(rename = "Document")]
    remt_001_001_05(iso_20022_remt::remt_001_001_05::Document<Dmkr>),
    #[serde(rename = "Document")]
    remt_002_001_02(iso_20022_remt::remt_002_001_02::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // remt
            "remt.001.001.05" => Document::remt_001_001_05(Default::default()),
            "remt.002.001.02" => Document::remt_002_001_02(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}

#[cfg(test)]
pub mod tests {
    use chrono::Utc;

    use crate::documents::{remt::remt_001_001_05, Dmkr};

    pub fn party_identification_135() -> Result<
        remt_001_001_05::PartyIdentification135,
        remt_001_001_05::PartyIdentification135BuilderError,
    > {
        let mut builder = remt_001_001_05::PartyIdentification135Builder::default();

        builder.nm(Some(remt_001_001_05::Max140Text {
            value: "Emergent Financial, LLC".into(),
        }));

        builder.pstl_adr(Some(remt_001_001_05::PostalAddress24 {
            adr_tp: Some(remt_001_001_05::AddressType3Choice {
                value: remt_001_001_05::AddressType3ChoiceEnum {
                    cd: Some(remt_001_001_05::AddressType2Code::Pbox),
                    prtry: None,
                },
            }),
            strt_nm: Some(remt_001_001_05::Max70Text {
                value: "1102 A St. #741".into(),
            }),
            ctry: Some(remt_001_001_05::CountryCode { value: "US".into() }),
            ..Default::default()
        }));

        builder.build()
    }

    #[test]
    fn test_remt_001_001_05() -> Result<(), String> {
        let mut doc = remt_001_001_05::DocumentBuilder::<Dmkr>::default();

        doc.rmt_advc(remt_001_001_05::RemittanceAdviceV05 {
            grp_hdr: remt_001_001_05::GroupHeader79 {
                msg_id: remt_001_001_05::Max35Text {
                    value: "1ca49e14-3fc6-403f-9a52-bf62c992dfd7".into(),
                },
                cre_dt_tm: remt_001_001_05::IsoDateTime { value: Utc::now() },
                authstn: vec![remt_001_001_05::Authorisation1Choice {
                    value: remt_001_001_05::Authorisation1ChoiceEnum {
                        cd: Some(remt_001_001_05::Authorisation1Code::Auth),
                        prtry: None,
                    },
                }],
                cpy_ind: None,
                initg_pty: remt_001_001_05::PartyIdentification135 {
                    nm: Some(remt_001_001_05::Max140Text {
                        value: "Emergent Financial, LLC".into(),
                    }),
                    pstl_adr: Some(remt_001_001_05::PostalAddress24 {
                        adr_tp: Some(remt_001_001_05::AddressType3Choice {
                            value: remt_001_001_05::AddressType3ChoiceEnum {
                                cd: Some(remt_001_001_05::AddressType2Code::Pbox),
                                prtry: None,
                            },
                        }),
                        strt_nm: Some(remt_001_001_05::Max70Text {
                            value: "1102 A St. #741".into(),
                        }),
                        ctry: Some(remt_001_001_05::CountryCode { value: "US".into() }),
                        ..Default::default()
                    }),
                    id: Some(remt_001_001_05::Party38Choice {
                        value: remt_001_001_05::Party38ChoiceEnum {
                            org_id: Some(remt_001_001_05::OrganisationIdentification29 {
                                any_bic: Some(remt_001_001_05::AnyBicDec2014Identifier {
                                    value: "EMFGUS66".into(),
                                }),
                                ..Default::default()
                            }),
                            prvt_id: None,
                        },
                    }),
                    ctry_of_res: Some(remt_001_001_05::CountryCode { value: "US".into() }),
                    ctct_dtls: Some(remt_001_001_05::Contact4 {
                        nm_prfx: None,
                        nm: Some(remt_001_001_05::Max140Text {
                            value: "Ryan Tate".into(),
                        }),
                        phne_nb: None,
                        mob_nb: Some(remt_001_001_05::PhoneNumber {
                            value: "+1-206-519-2818".into(),
                        }),
                        fax_nb: None,
                        email_adr: Some(remt_001_001_05::Max2048Text {
                            value: "ryan.tate@emergent.financial".into(),
                        }),
                        email_purp: None,
                        job_titl: Some(remt_001_001_05::Max35Text {
                            value: "CEO".into(),
                        }),
                        rspnsblty: None,
                        dept: None,
                        othr: vec![],
                        prefrd_mtd: Some(remt_001_001_05::PreferredContactMethod1Code::Mail),
                    }),
                },
                msg_rcpt: None,
                fwdg_agt: None,
            },
            rmt_inf: vec![],
            splmtry_data: vec![],
        });

        // let doc = Document::from_namespace("remt.001.001.05");

        // let xml_string = to_string(&doc).expect("failed to serialize document");

        // println!("xml_string: {}", xml_string);

        // let doc2 = from_str::<remt::Document>(&xml_string).expect("failed to deserialize document");

        // println!("doc2: {:?}", doc2);

        Ok(())
    }
}
