# ISO 20022 SDK

[![Rust](https://img.shields.io/badge/Language-Rust-blue.svg)](https://www.rust-lang.org/)
[![ISO 20022](https://img.shields.io/badge/ISO-20022-informational.svg)](https://www.iso20022.org/)

This repository is an **ISO 20022 SDK** written in **Rust**, designed to provide a high-performance and type-safe way to handle financial messages based on the ISO 20022 standard. The project is currently in development, and its goal is to facilitate message creation, validation, and parsing, enabling financial applications to integrate smoothly with systems using ISO 20022-compliant messages.

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Usage](#usage)
  - [Message Creation](#message-creation)
  - [Message Parsing](#message-parsing)
- [Supported Message Types](#supported-message-types)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

## Overview

ISO 20022 is a global standard for electronic data interchange between financial institutions. This SDK aims to implement the ISO 20022 message structure in Rust to ensure:

- **Performance**: Leveraging Rust’s system-level capabilities to process large volumes of financial data.
- **Safety**: Rust's strong type system ensures safe memory management and prevents common runtime errors.
- **Flexibility**: Modular and extensible to support a wide range of ISO 20022 message types.

> **Note**: This SDK is still under active development. New message types and features are being added regularly.

## Installation

To include this SDK in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
iso-20022-sdk = { git = "https://github.com/RustLangLatam/iso-20022-sdk" }
```

You can now run `cargo build` to fetch the SDK and its dependencies.

## Usage

### Message Creation

The SDK allows you to build ISO 20022 messages through a builder pattern. Below is an example of creating a `pacs.008.001.10` message, which is typically used for customer credit transfer messages.

```rust
use iso_20022_sdk::primitives::{BicfiDec2014Identifier, GenericOrganisationIdentification1};
use iso_20022_sdk::types::{
    Max35Text, OrganisationIdentification29, FinancialInstitutionIdentification18, Document, Message,
};

fn main() {
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

    println!("Document: {:#?}", msg);

    let xml = msg.to_xml().unwrap();
    println!("Generated XML: {}", xml);
}
```

In this example, we create a `pacs.008` message with necessary identifiers such as `Business Message Identifier`, `Recipient Organisation`, `Sender Organisation`, and `Sender Financial Institution`. The message is then serialized into XML format.

### Message Parsing

The SDK also allows you to parse ISO 20022 messages from an XML file, which can then be used within your Rust application. Here's an example of parsing a `pacs.008` message:

```rust
use iso_20022_sdk::types::Message;

fn main() {
    let file_content = std::fs::read_to_string("test/pacs.008.001.10.xml")
        .expect("Unable to read file");

    let parsed_msg = Message::<Dmkr>::from_xml(&file_content).unwrap();
    
    println!("Parsed Message: {:?}", parsed_msg.app_hdr());
}
```

This example demonstrates how you can read an XML file containing a `pacs.008.001.10` message and parse it into the corresponding Rust structure. The parsed message can then be used for further processing.

## Supported Message Types

The SDK currently supports the following ISO 20022 message types:

- **PAIN (Payment Initiation)**: 
  - `pain.001` – Customer Credit Transfer Initiation.
  - `pain.002` – Status Report.

- **CAMT (Cash Management)**: 
  - `camt.052` – Bank to Customer Account Report.
  - `camt.053` – Bank to Customer Statement.
  - `camt.054` – Bank to Customer Debit/Credit Notification.

- **PACS (Payments Clearing and Settlement)**: 
  - `pacs.008` – FI to FI Customer Credit Transfer.

> **Note**: More message types will be supported in future releases. We are actively working on extending the message library.

## Roadmap

We are working towards full compliance with the ISO 20022 standard. Here are some upcoming features:

- **Support for additional message types** (e.g., `pacs.004`, `pacs.009`).
- **Improved validation mechanisms** using external XML schemas.
- **Extensive documentation and usage guides** for each message type.
- **Integration with external APIs** for message transmission.

Please refer to the [issues](https://github.com/RustLangLatam/iso-20022-sdk/issues) section for ongoing work and planned features.

## Contributing

We welcome contributions to this SDK! Whether it's adding new features, fixing bugs, or improving the documentation, your help is appreciated. To contribute:

1. Fork the repository.
2. Create a new branch for your feature (`git checkout -b feature/my-feature`).
3. Commit your changes (`git commit -m "Add my new feature"`).
4. Push to the branch (`git push origin feature/my-feature`).
5. Open a Pull Request.

Please ensure your code adheres to the Rust coding standards and is well-documented.

## License

This project is licensed under the MIT License. For more details, see the [LICENSE](LICENSE) file.

---

This SDK is part of an ongoing effort to modernize and streamline the adoption of the ISO 20022 messaging standard in the financial industry, with the added benefit of Rust’s safety and performance features.
