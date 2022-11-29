#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rpc {
    #[prost(message, repeated, tag="1")]
    pub subscriptions: ::prost::alloc::vec::Vec<rpc::SubOpts>,
    #[prost(message, repeated, tag="2")]
    pub publish: ::prost::alloc::vec::Vec<Message>,
}
/// Nested message and enum types in `RPC`.
pub mod rpc {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubOpts {
        /// subscribe or unsubcribe
        #[prost(bool, optional, tag="1")]
        pub subscribe: ::core::option::Option<bool>,
        #[prost(string, optional, tag="2")]
        pub topic_id: ::core::option::Option<::prost::alloc::string::String>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(bytes="vec", optional, tag="1")]
    pub from: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="3")]
    pub seqno: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag="4")]
    pub topic_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
