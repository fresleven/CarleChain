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
    #[prost(bytes="vec", optional, tag="5")]
    pub signature: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="6")]
    pub key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
