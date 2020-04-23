//! `sage_mqtt` is a an encoding/decoding library for MQTT 5.0 protocol.
//! The library consists in pivot types, such as `UTF8String` that can be
//! written to and read from a stream as well as converted to standard Rust
//! types.
#[allow(unused_macros)]
macro_rules! assert_matches {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )?) => {
        assert!(matches!($expression, $( $pattern )|+ $( if $guard )?))
    }
}

mod broker;
mod codec;
mod control_packets;
mod error;
mod quality_of_service;
mod reason_code;

pub use broker::Broker;
use codec::{
    ReadBinaryData, ReadByte, ReadFourByteInteger, ReadTwoByteInteger, ReadUTF8String,
    ReadVariableByteInteger, WriteBinaryData, WriteByte, WriteFourByteInteger, WriteTwoByteInteger,
    WriteUTF8String, WriteVariableByteInteger,
};
pub use control_packets::{
    Auth, Authentication, ConnAck, Connect, ControlPacket, Disconnect, PubAck, PubComp, PubRec,
    PubRel, Publish, RetainHandling, SubAck, Subscribe, SubscriptionOptions, UnSubAck, UnSubscribe,
};
use control_packets::{
    ControlPacketType, FixedHeader, PropertiesDecoder, Property, PropertyId, DEFAULT_MAXIMUM_QOS,
    DEFAULT_PAYLOAD_FORMAT_INDICATOR, DEFAULT_RECEIVE_MAXIMUM, DEFAULT_REQUEST_PROBLEM_INFORMATION,
    DEFAULT_REQUEST_RESPONSE_INFORMATION, DEFAULT_RETAIN_AVAILABLE,
    DEFAULT_SESSION_EXPIRY_INTERVAL, DEFAULT_SHARED_SUBSCRIPTION_AVAILABLE,
    DEFAULT_TOPIC_ALIAS_MAXIMUM, DEFAULT_WILCARD_SUBSCRIPTION_AVAILABLE,
    DEFAULT_WILL_DELAY_INTERVAL,
};
pub use error::{Error, Result};
pub use quality_of_service::QoS;
pub use reason_code::ReasonCode;
