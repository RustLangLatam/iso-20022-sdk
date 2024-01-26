use std::io::BufReader;

use chrono::NaiveDateTime;
use sxd_document::parser;
use sxd_xpath::evaluate_xpath;
use tracing::debug;
use xml::{EventReader, reader::XmlEvent};

use crate::crypto::Signature;
use crate::documents::{Dmkr, Document};
use crate::models::head::head_001_001_03::self as head;
use crate::models::nvlp::nvlp_001_001_01::self as nvlp;
use crate::types::{BranchAndFinancialInstitutionIdentification6, FinancialInstitutionIdentification18, ISODateTime, Max35Text, OrganisationIdentification29, Party38Choice, PartyIdentification135, PersonIdentification13};

/// Default Envelope Type
pub type DefaultMsgEnvlp =
nvlp::BizMsgEnvlp<head::AppHdr<Signature, Signature>, Document, Dmkr, Dmkr>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error Serializing / Deserializing XML
    #[error(transparent)]
    XmlSerDe(#[from] quick_xml::de::DeError),
    /// SXD Document Error
    #[error(transparent)]
    XsdDocument(#[from] sxd_document::parser::Error),
    /// SXD XPath Error
    #[error(transparent)]
    XsdXPath(#[from] sxd_xpath::Error),
    /// Signing Error
    #[error("data store disconnected")]
    Signing(signature::Error),
}

#[derive(Debug, Clone, Default)]
pub struct Message<
    'a,
    Doc: std::fmt::Debug
    + Default
    + Clone
    + PartialEq
    + ::serde::Serialize
    + ::serde::Deserialize<'a>
    + ::validator::Validate,
> {
    /// XML string representing the inner type. Used internally to parse the inner type.
    /// An incoming message will use this field for helping to determine what the
    /// inner type is.
    ///
    /// use the `to_xml()` method to get the XML string representation of the message
    /// inner type.
    pub xml_string: &'a str,
    /// Internal representation of the message envelope
    pub inner: nvlp::BizMsgEnvlp<head::AppHdr<Signature, Signature>, Doc, Dmkr, Dmkr>,
}

impl<'a, Doc> Message<'a, Doc>
    where
        Doc: std::fmt::Debug
        + Default
        + Clone
        + PartialEq
        + ::serde::Serialize
        + ::serde::Deserialize<'a>
        + ::validator::Validate,
{
    pub fn builder() -> Self {
        let envlp = Self::default();

        // Automatically set the envlp and header namespaces
        envlp.set_namespace()
    }

    /// Return the application header from the message envelope
    pub fn app_hdr(&self) -> Option<head::AppHdr<Signature, Signature>> {
        self.inner.value.hdr.clone().map(|hdr| hdr.value)
    }

    /// Set the application header AppHdr of the message
    /// Note, this will overwrite the existing AppHdr
    pub fn set_app_hdr(self, app_hdr: head::AppHdr<Signature, Signature>) -> Self {
        let mut msg = self;

        // Set the AppHdr
        msg.inner.value.hdr = Some(nvlp::LaxPayload { value: app_hdr });

        msg
    }

    /// Set the recipient of the message
    pub fn set_recipient(self, recipient: head::Party44Choice) -> Self {
        let mut app_hdr = self.app_hdr().unwrap_or_default();
        app_hdr.value.to = recipient;

        self.set_app_hdr(app_hdr)
    }
    //
    // /// Set the recipient organization id of the message.
    // /// This is a simplified version of `set_recipient` that only takes an organization id.
    // /// Use the `set_recipient_fi_id()` method to set a financial institutiton id.
    // /// Note, this will overwrite any existing recipient.
    // pub fn set_recipient_org_id(self, org_id: OrganisationIdentification29) -> Self {
    //     self.set_recipient(head::Party44Choice {
    //         value: head::Party44ChoiceEnum {
    //             org_id: Some(PartyIdentification135 {
    //                 id: Some(Party38Choice {
    //                     value: Party38ChoiceEnum {
    //                         org_id: Some(org_id),
    //                         ..Default::default()
    //                     },
    //                 }),
    //                 ..Default::default()
    //             }),
    //             ..Default::default()
    //         },
    //     })
    // }

    /// Set the recipient financial institution id of the message.
    /// This is a simplified version of `set_recipient` that only takes a financial institution id.
    /// Use the `set_recipient_org_id()` method to set an organization id.
    /// Note, this will overwrite any existing recipient.
    pub fn set_recipient_fi_id(self, fin_instn_id: FinancialInstitutionIdentification18) -> Self {
        self.set_recipient(head::Party44Choice {
            value: head::Party44ChoiceEnum {
                fi_id: Some(BranchAndFinancialInstitutionIdentification6 {
                    fin_instn_id,
                    ..Default::default()
                }),
                ..Default::default()
            },
        })
    }

    // /// Set the recipient private individual id of the message.
    // /// This is a simplified version of `set_recipient` that only takes a private individual id.
    // /// Use the `set_recipient_org_id()` method to set an organization id or the `set_recipient_fi_id()`
    // /// method to set a financial institution id.
    // /// Note, this will overwrite any existing recipient.
    // pub fn set_recipient_prvt_id(self, prvt_id: PersonIdentification13) -> Self {
    //     self.set_recipient(head::Party44Choice {
    //         value: head::Party44ChoiceEnum {
    //             org_id: Some(PartyIdentification135 {
    //                 id: Some(Party38Choice {
    //                     value: Party38ChoiceEnum {
    //                         prvt_id: Some(prvt_id),
    //                         ..Default::default()
    //                     },
    //                 }),
    //                 ..Default::default()
    //             }),
    //             ..Default::default()
    //         },
    //     })
    // }

    /// Set the sender of the message
    pub fn set_sender(self, sender: head::Party44Choice) -> Self {
        let mut app_hdr = self.app_hdr().unwrap_or_default();
        app_hdr.value.fr = sender;

        self.set_app_hdr(app_hdr)
    }

    // /// Set the sender organization id of the message.
    // /// This is a simplified version of `set_sender` that only takes an organization id.
    // /// Use the `set_sender_fi_id()` method to set a financial institutiton id.
    // /// Note, this will overwrite any existing sender.
    // pub fn set_sender_org_id(self, org_id: OrganisationIdentification29) -> Self {
    //     self.set_sender(head::Party44Choice {
    //         value: head::Party44ChoiceEnum {
    //             org_id: Some(PartyIdentification135 {
    //                 id: Some(Party38Choice {
    //                     value: Party38ChoiceEnum {
    //                         org_id: Some(org_id),
    //                         ..Default::default()
    //                     },
    //                 }),
    //                 ..Default::default()
    //             }),
    //             ..Default::default()
    //         },
    //     })
    // }

    /// Set the sender financial institution id of the message.
    /// This is a simplified version of `set_sender` that only takes a financial institution id.
    /// Use the `set_sender_org_id()` method to set an organization id.
    /// Note, this will overwrite any existing sender.
    pub fn set_sender_fi_id(self, fin_instn_id: FinancialInstitutionIdentification18) -> Self {
        self.set_sender(head::Party44Choice {
            value: head::Party44ChoiceEnum {
                fi_id: Some(BranchAndFinancialInstitutionIdentification6 {
                    fin_instn_id,
                    ..Default::default()
                }),
                ..Default::default()
            },
        })
    }

    // /// Set the sender private individual id of the message.
    // /// This is a simplified version of `set_sender` that only takes a private individual id.
    // /// Use the `set_sender_org_id()` method to set an organization id or the `set_sender_fi_id()`
    // /// method to set a financial institution id.
    // /// Note, this will overwrite any existing sender.
    // pub fn set_sender_prvt_id(self, prvt_id: PersonIdentification13) -> Self {
    //     self.set_sender(head::Party44Choice {
    //         value: head::Party44ChoiceEnum {
    //             org_id: Some(PartyIdentification135 {
    //                 id: Some(Party38Choice {
    //                     value: Party38ChoiceEnum {
    //                         prvt_id: Some(prvt_id),
    //                         ..Default::default()
    //                     },
    //                 }),
    //                 ..Default::default()
    //             }),
    //             ..Default::default()
    //         },
    //     })
    // }

    /// e.g. `Document`
    pub fn set_biz_msg_idr(self, idr: Max35Text) -> Self {
        let mut app_hdr = self.app_hdr().unwrap_or_default();
        app_hdr.value.biz_msg_idr = idr;

        self.set_app_hdr(app_hdr)
    }

    /// e.g. `pacs.008.001.07`
    pub fn set_msg_def_idr(self, idr: Max35Text) -> Self {
        let mut app_hdr = self.app_hdr().unwrap_or_default();
        app_hdr.value.msg_def_idr = idr;

        self.set_app_hdr(app_hdr)
    }

    /// Set the created date time of the message.
    /// This will be set to the current UTC time.
    pub fn set_cre_dt(self) -> Self {
        let mut app_hdr = self.app_hdr().unwrap_or_default();
        app_hdr.value.cre_dt = ISODateTime {
            value: NaiveDateTime::default(),
        };

        self.set_app_hdr(app_hdr)
    }

    /// Set the xml namespace of the message and business header.
    pub fn set_namespace(self) -> Self {
        let mut envlp = self;

        // Set the envelope namespace
        envlp.inner.value.xmlns = nvlp::namespace();

        // Set the header namespace
        let mut app_hdr = envlp.app_hdr().unwrap_or_default();
        app_hdr.value.xmlns = head::namespace();

        envlp.set_app_hdr(app_hdr)
    }

    /// Set the document of the message.
    /// Note, the document must set its own namespace value.
    /// By default, all root iso-20022 message documents have
    /// an attribute field, `xmlns`, that is used to set the document namespace.
    /// The document namespace must be set before calling this method.
    pub fn set_document(self, doc: Doc) -> Self {
        let mut envlp = self;
        envlp.inner.value.doc.value = doc;

        envlp
    }

    /// Sign the document at an optional xpath, e.g. `/Document/AcctOpngInstr`
    /// If no xpath is provided, the entire document will be signed, e.g. `/Document`
    /// Note, this will overwrite any existing signature.
    /// ```rust
    /// use iso_20022_sdk::prelude::*;
    ///
    /// let msg = Message::<_>::builder()
    ///     // Document must be set before signing
    ///     .set_document(doc)
    ///     // Sign the entire document
    ///     .sign_document(&signer, None);
    ///
    ///
    /// ```
    pub fn sign_document(
        self,
        signer: impl signature::Signer<Signature>,
        xpath: Option<&str>,
    ) -> Result<Self, Error> {
        let xml = quick_xml::se::to_string(&self.document())?;
        let package = parser::parse(&xml)?;
        let doc = package.as_document();

        // By default, sign the entire document
        let xpath = xpath.unwrap_or("/Document");
        let data = evaluate_xpath(&doc, xpath)?.into_string();

        // TODO, hash the data before signing
        let data = data.as_bytes();

        // Sign the xpath data
        let signature = signer.try_sign(data).unwrap();

        // Set the signature in the application header
        let mut app_hdr = self.app_hdr().unwrap_or_default();
        app_hdr.value.sgntr = Some(head::SignatureEnvelope { value: signature });

        // Set the application header and return the envelope
        Ok(self.set_app_hdr(app_hdr))
    }

    /// Set the related business reference of the message.
    pub fn set_rltd(self, rltd: head::BusinessApplicationHeader7<Signature>) -> Self {
        let mut app_hdr = self.app_hdr().unwrap_or_default();
        app_hdr.value.rltd.push(rltd);

        self.set_app_hdr(app_hdr)
    }

    /// Return the envelope document.
    pub fn document(&self) -> Doc {
        self.inner.value.doc.value.clone()
    }

    /// Return the serialized xml string of the inner type.
    pub fn to_xml(&self) -> Result<String, Error> {
        let xml_string = quick_xml::se::to_string(&self.inner)?;

        Ok(xml_string)
    }

    /// parse the header from the xml string
    pub fn from_xml(xml_str: &'a str) -> Result<Self, Error> {
        let inner = quick_xml::de::from_str(xml_str)?;

        println!("value: {:?}", inner);

        let mut msg = Self {
            xml_string: xml_str,
            inner,
        };

        // Parse the msg into the inner type;
        msg.parse()?;

        Ok(msg)
    }

    fn parse(&mut self) -> Result<(), Error> {
        // Use xml-reader to parse the xml string and find the `MsgDefIdr` element in the `head.001.001.03` namespace.
        let buf_reader = BufReader::new(self.xml_string.as_bytes());
        let event_reader = EventReader::new(buf_reader);

        for e in event_reader {
            match e {
                Ok(XmlEvent::ProcessingInstruction { name, data }) => {
                    println!("ProcessingInstruction::name: {}", name);
                    println!("ProcessingInstruction::data: {:?}", data);
                }
                Ok(XmlEvent::StartElement {
                       name:
                       xml::name::OwnedName {
                           local_name,
                           namespace,
                           ..
                       },
                       ..
                   }) => {
                    println!("StartElement::local_name: {}", local_name);
                    println!("StartElement::namespace: {:?}", namespace);
                }
                Ok(XmlEvent::Characters(data)) => {
                    debug!("Characters::data: {}", data);
                }
                _ => (),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::{BicfiDec2014Identifier, GenericOrganisationIdentification1};

    use super::*;

    #[test]
    fn test_message_builder() {
        let msg = Message::<_>::builder()
            .set_cre_dt()
            .set_msg_def_idr(Max35Text {
                value: "pacs.008.001.10".to_string(),
            })
            .set_biz_msg_idr(Max35Text {
                value: "12312312312".to_string(),
            })
            .set_recipient_org_id(OrganisationIdentification29 {
                othr: vec![GenericOrganisationIdentification1 {
                    id: Max35Text {
                        value: "2342342342".to_string(),
                    },
                    issr: Some(Max35Text {
                        value: "FOOISSUER".to_string(),
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            })
            .set_sender_org_id(OrganisationIdentification29 {
                othr: vec![GenericOrganisationIdentification1 {
                    id: Max35Text {
                        value: "b3033215-3a30-48ee-b194-5c02e08a5fb3".to_string(),
                    },
                    ..Default::default()
                }],
                ..Default::default()
            })
            .set_sender_fi_id(FinancialInstitutionIdentification18 {
                bicfi: Some(BicfiDec2014Identifier {
                    value: "ABCDUS33XXX".to_string(),
                }),
                ..Default::default()
            })
            .set_document(Document::from_namespace("pacs.008.001.10"));

        println!("DOC: {:#?}", msg);

        let xml = msg.to_xml().unwrap();
        //
        // println!("xml: {}", xml);

        assert!(false)
    }

    #[test]
    fn test_parse_message() {
        let file =
            std::fs::read_to_string("test/pacs.008.001.10.xml").expect("Unable to read file");

        let _msg = Message::<Dmkr>::from_xml(&file).unwrap();

        println!("###: {:?}", _msg.app_hdr());

        assert!(false)
    }
}
