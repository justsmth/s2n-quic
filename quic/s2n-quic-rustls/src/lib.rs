// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

/// *WARNING*: These are deprecated and should not be used.
#[deprecated = "client and server builders should be used instead"]
pub use ::rustls::{Certificate, PrivateKey};

#[deprecated = "client and server builders should be used instead"]
pub mod rustls {
    pub use ::rustls::*;
}

/// Wrap error types in Box to avoid leaking rustls types
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

mod cipher_suite;
mod error;
mod session;

pub mod certificate;
pub mod client;
pub mod server;

#[deprecated = "client and server builders should be used instead"]
pub static DEFAULT_CIPHERSUITES: &[rustls::SupportedCipherSuite] =
    cipher_suite::DEFAULT_CIPHERSUITES;

pub use client::Client;
pub use server::Server;

//= https://www.rfc-editor.org/rfc/rfc9001#section-4.2
//# Clients MUST NOT offer TLS versions older than 1.3.
static PROTOCOL_VERSIONS: &[&rustls::SupportedProtocolVersion] = &[&rustls::version::TLS13];

/// The supported version of quic
const QUIC_VERSION: rustls::quic::Version = rustls::quic::Version::V1;

#[cfg(test)]
mod tests {
    use super::*;
    use s2n_quic_core::crypto::tls::{self, testing::certificates::*};

    #[test]
    fn client_server_test() {
        let mut client = client::Builder::new()
            .with_certificate(CERT_PEM)
            .unwrap()
            .build()
            .unwrap();

        let mut server = server::Builder::new()
            .with_certificate(CERT_PEM, KEY_PEM)
            .unwrap()
            .build()
            .unwrap();

        let mut pair = tls::testing::Pair::new(&mut server, &mut client, "localhost".into());

        while pair.is_handshaking() {
            pair.poll(None).unwrap();
        }

        pair.finish();
    }
}
