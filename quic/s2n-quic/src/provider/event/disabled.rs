// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::sync::atomic::AtomicU64;

use crate::provider::event::{ConnectionInfo, ConnectionMeta};

#[derive(Debug, Default)]
pub struct Provider;

impl super::Provider for Provider {
    type Subscriber = Subscriber;
    type Error = core::convert::Infallible;

    fn start(self) -> Result<Self::Subscriber, Self::Error> {
        Ok(Subscriber)
    }
}

#[derive(Default, Debug)]
pub struct AckContext {
    ack_tx: AtomicU64,
    ack_rx: AtomicU64,
}

pub struct Subscriber;

impl super::Subscriber for Subscriber {
    type ConnectionContext = AckContext;

    fn create_connection_context(
        &mut self,
        _meta: &ConnectionMeta,
        _info: &ConnectionInfo,
    ) -> Self::ConnectionContext {
        Default::default()
    }

    fn on_frame_sent(
        &mut self,
        context: &mut Self::ConnectionContext,
        _meta: &ConnectionMeta,
        event: &s2n_quic_core::event::api::FrameSent,
    ) {
        if let s2n_quic_core::event::api::Frame::Ack { .. } = event.frame {
            context
                .ack_tx
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }

    fn on_frame_received(
        &mut self,
        context: &mut Self::ConnectionContext,
        _meta: &ConnectionMeta,
        event: &s2n_quic_core::event::api::FrameReceived,
    ) {
        if let s2n_quic_core::event::api::Frame::Ack { .. } = event.frame {
            context
                .ack_rx
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }

    fn on_connection_closed(
        &mut self,
        context: &mut Self::ConnectionContext,
        _meta: &ConnectionMeta,
        _event: &s2n_quic_core::event::api::ConnectionClosed,
    ) {
        println!("----------ack context {:?}", context)
    }
}
