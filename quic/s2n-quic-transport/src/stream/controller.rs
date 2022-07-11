// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    connection::open_token,
    contexts::OnTransmitError,
    stream::legacy_controller::{MaxStreamsToFrameWriter, StreamsBlockedToFrameWriter},
    sync::{IncrementalValueSync, PeriodicSync},
    transmission::WriteContext,
};
use core::task::Waker;
use s2n_quic_core::{
    ack,
    stream::{StreamId, StreamType},
    time::Timestamp,
    varint::VarInt,
};
use smallvec::SmallVec;

// The amount of wakers that may be tracked before allocating to the heap.
const WAKERS_INITIAL_CAPACITY: usize = 5;

#[allow(unused)]
pub struct Controller {
    // local initiated
    //
    // max_local_limit: max_local_bidi_stream
    // peer_stream_limit: peer_max_bidi_stream
    //
    // limits: max_local_bidi_stream.min(peer_max_bidi_stream)
    // tranmit: streams_blocked
    bidi_local: LocalInitiatedStreamController,

    // local initiated (outgoing)
    //
    // max_local_limit: max_local_uni_stream
    // peer_stream_limit: peer_max_uni_stream
    //
    // limits: max_local_uni_stream.min(peer_max_uni_stream)
    // tranmit: streams_blocked
    uni_local: LocalInitiatedStreamController,

    // remote initiated
    //
    // max_local_limit: max_remote_bidi_stream (declared locally)
    //
    // tranmit: max_streams
    bidi_remote: RemoteInitiatedStreamController,

    // remote initiated (incoming)
    //
    // max_local_limit: max_remote_uni_stream (declared locally)
    //
    // tranmit: max_streams
    uni_remote: RemoteInitiatedStreamController,
}

#[allow(unused)]
pub struct LocalInitiatedStreamController {
    // the max stream limit specified by the local endpoint.
    //
    // max_local_limit.min(peer_stream_limit) is used to determine how
    // many local streams can be opened.
    max_local_limit: VarInt,

    // the cumulative stream limit specified by the peer endpoint.
    //
    // can be updated when MAX_STREAMS frame is received
    peer_stream_limit: VarInt,

    // opened_streams is needed to track the latest opened stream since
    // peer_stream_limit is a cumulative limit.
    opened_streams: VarInt,
    closed_streams: VarInt,

    streams_blocked_frame_sync: PeriodicSync<VarInt, StreamsBlockedToFrameWriter>,
    wakers: SmallVec<[Waker; WAKERS_INITIAL_CAPACITY]>,
    token_counter: open_token::Counter,
    expired_token: open_token::Token,
    stream_type: StreamType,
}

#[allow(unused)]
impl LocalInitiatedStreamController {
    fn new() -> Self {
        todo!()
    }
    fn poll_open_stream(&mut self) {}
    fn on_close_stream(&mut self) {}
    fn on_close(&mut self) {}

    fn on_timeout(&mut self, now: Timestamp) {}
    fn on_packet_ack<A: ack::Set>(&mut self, ack_set: &A) {}
    fn on_packet_loss<A: ack::Set>(&mut self, ack_set: &A) {}
    fn on_transmit<W: WriteContext>(
        &mut self,
        stream_id: StreamId,
        context: &mut W,
    ) -> Result<(), OnTransmitError> {
        Ok(())
    }
    fn on_max_streams_frame(&self) {}
}

#[allow(unused)]
struct RemoteInitiatedStreamController {
    // the max stream limit specified by the local endpoint.
    //
    // used to calculate updated max_streams_frame_sync value as the peer
    // closes streams.
    max_local_limit: VarInt,

    // responsible for advertising updated max stream frames as the
    // peer closes streams
    max_streams_frame_sync: IncrementalValueSync<VarInt, MaxStreamsToFrameWriter>,

    // opened_streams: VarInt, // only used for checks.. maybe add later
    closed_streams: VarInt,
    stream_type: StreamType,
}

#[allow(unused)]
impl RemoteInitiatedStreamController {
    fn new() -> Self {
        todo!()
    }
    fn on_open_stream(&self) {}
    fn on_close_stream(&mut self) {}
    fn on_close(&mut self) {}

    fn on_timeout(&mut self, now: Timestamp) {}
    fn on_packet_ack<A: ack::Set>(&mut self, ack_set: &A) {}
    fn on_packet_loss<A: ack::Set>(&mut self, ack_set: &A) {}
    fn on_transmit<W: WriteContext>(
        &mut self,
        stream_id: StreamId,
        context: &mut W,
    ) -> Result<(), OnTransmitError> {
        Ok(())
    }
    fn on_max_streams_frame(&self) {}
}

//=======================================
//
//#[derive(Debug)]
//struct LocalInitiatedController {
//    local_initiated_concurrent_stream_limit: VarInt,
//    peer_cumulative_stream_limit: VarInt,
//    wakers: SmallVec<[Waker; WAKERS_INITIAL_CAPACITY]>,
//    streams_blocked_sync: PeriodicSync<VarInt, StreamsBlockedToFrameWriter>,
//    opened_streams: VarInt,
//    closed_streams: VarInt,
//    /// Keeps track of all of the issued open tokens
//    token_counter: open_token::Counter,
//    /// Keeps track of all of the expired open tokens
//    expired_token: open_token::Token,
//}

//struct RemoteInitiatedController {
//    peer_initiated_concurrent_stream_limit: VarInt,
//    max_streams_sync: IncrementalValueSync<VarInt, MaxStreamsToFrameWriter>,
//    opened_streams: VarInt,
//    closed_streams: VarInt,
//}

//=======================================
//
// impl timer::Provider for Controller {
// impl transmission::interest::Provider for Controller {

// impl timer::Provider for ControllerPair {
// impl transmission::interest::Provider for ControllerPair {

// impl timer::Provider for OutgoingController {
// impl transmission::interest::Provider for OutgoingController {
//      self.streams_blocked_sync.transmission_interest(query)

// impl transmission::interest::Provider for IncomingController {
//      self.max_streams_sync.transmission_interest(query)

// #[derive(Debug, Default)]
// pub(super) struct StreamsBlockedToFrameWriter {}
// impl ValueToFrameWriter<VarInt> for StreamsBlockedToFrameWriter {

// #[derive(Debug, Default)]
// pub(super) struct MaxStreamsToFrameWriter {}
// impl ValueToFrameWriter<VarInt> for MaxStreamsToFrameWriter {
