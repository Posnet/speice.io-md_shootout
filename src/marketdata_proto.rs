#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    #[prost(uint64, tag="1")]
    pub price: u64,
    #[prost(uint32, tag="2")]
    pub size: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LevelUpdate {
    #[prost(uint64, tag="1")]
    pub price: u64,
    #[prost(uint32, tag="2")]
    pub size: u32,
    #[prost(uint32, tag="3")]
    pub flags: u32,
    #[prost(enumeration="level_update::Side", tag="4")]
    pub side: i32,
}
pub mod level_update {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Side {
        Buy = 0,
        Sell = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(int64, tag="1")]
    pub ts_nanos: i64,
    #[prost(string, tag="2")]
    pub symbol: std::string::String,
    #[prost(oneof="message::MessageOneof", tags="3, 4")]
    pub message_oneof: ::std::option::Option<message::MessageOneof>,
}
pub mod message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MessageOneof {
        #[prost(message, tag="3")]
        Trade(super::Trade),
        #[prost(message, tag="4")]
        Level(super::LevelUpdate),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiMessage {
    #[prost(uint64, tag="1")]
    pub seqno: u64,
    #[prost(message, repeated, tag="2")]
    pub messages: ::std::vec::Vec<Message>,
}
