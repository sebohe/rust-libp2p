#![no_main]
use libfuzzer_sys::fuzz_target;
use libp2p_gossipsub::{protocol::GossipsubCodec, ValidationMode, HandlerEvent};
use bytes::BytesMut;
use unsigned_varint::codec;
use futures_codec::{Decoder, Encoder};

fuzz_target!(|data: &[u8]| {
    let mut codec = GossipsubCodec::new(codec::UviBytes::default(), ValidationMode::Strict);
    let mut buf: BytesMut = data.into();
    if let Ok(rpc) = codec.decode(&mut buf) {
        if let Some(rpc) = rpc {
            match rpc {
                HandlerEvent::Message { rpc, .. } => {
                    let mut buf = BytesMut::new();
                    codec.encode(rpc, &mut buf);
                }
                _ => {},
            }
        }
    }
});
