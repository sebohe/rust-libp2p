#![no_main]
use bytes::BytesMut;
use futures_codec::Decoder;
use libfuzzer_sys::fuzz_target;
use libp2p_core::{connection::ConnectionId, PeerId};
use libp2p_gossipsub::{
    protocol::GossipsubCodec, Gossipsub, GossipsubConfigBuilder, HandlerEvent, MessageAuthenticity,
    PeerKind, ValidationMode,
};
use libp2p_swarm::NetworkBehaviour;
use unsigned_varint::codec;

fuzz_target!(|data: &[u8]| {
    let mut codec = GossipsubCodec::new(codec::UviBytes::default(), ValidationMode::Anonymous);
    let mut buf: BytesMut = data.into();
    if let Ok(rpc) = codec.decode(&mut buf) {
        if let Some(rpc) = rpc {
            // We have a valid HandlerEvent attempt to process the rpc
            let mut gs = create_gossipsub();

            // connect peer and set kind
            let peer = PeerId::random(); // TODO: Maybe it's better not to use random for fuzzing.
            gs.inject_connected(&peer);
            gs.inject_event(
                peer.clone(),
                ConnectionId::new(0),
                HandlerEvent::PeerKind(PeerKind::Gossipsubv1_1),
            );

            // Now inject our fuzz generated rpc
            gs.inject_event(peer.clone(), ConnectionId::new(0), rpc);
        }
    }
});

// Create a Gossipsub and connect a peer
fn create_gossipsub() -> Gossipsub {
    let mut gs_config_builder = GossipsubConfigBuilder::new();
    gs_config_builder.validation_mode(ValidationMode::Anonymous);
    let gs_config = gs_config_builder.build().unwrap();
    Gossipsub::new(MessageAuthenticity::Anonymous, gs_config).unwrap()
}
