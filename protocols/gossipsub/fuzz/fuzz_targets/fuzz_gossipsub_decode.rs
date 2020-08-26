#![no_main]
use bytes::BytesMut;
use futures_codec::{Decoder, Encoder};
use libfuzzer_sys::fuzz_target;
use libp2p_gossipsub::{protocol::GossipsubCodec, HandlerEvent, ValidationMode};
use unsigned_varint::codec;

fuzz_target!(|data: &[u8]| {
    let mut codec = GossipsubCodec::new(codec::UviBytes::default(), ValidationMode::Anonymous);
    let mut buf: BytesMut = data.into();
    if let Ok(rpc) = codec.decode(&mut buf) {
        if let Some(rpc) = rpc {
            match rpc {
                HandlerEvent::Message { rpc, .. } => {
                    let mut buf = BytesMut::new();
                    codec.encode(rpc, &mut buf);
                }
                _ => {}
            }
        }
    }
});
